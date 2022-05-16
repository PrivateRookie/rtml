use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tag_fmt::TagFormatter;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, HtmlElement, Window};

mod basic_impl;
mod events;
/// html tags
pub mod tags;
pub use events::*;
use tags::{Attrs, Content, Styles};
mod attr;
mod style;
pub use rtml_macro::page;
/// print template as html, instead of create them by dom api
pub mod tag_fmt;

pub trait Template {
    /// provide tag name, content, attr, styles and listeners
    /// to create html element
    fn resources(
        &self,
    ) -> (
        &'static str,
        &Attrs,
        &Styles,
        &Content,
        HashMap<&str, Box<dyn FnOnce() -> Box<dyn FnMut()> + '_>>,
    );

    /// after calling dom api to create element,
    /// replace marker ele field with real element
    fn set_element(&self, element: Element);

    /// generate html element and add event bindings
    fn render(&self, parent: &Element, doc: &Document) -> Result<Element, JsValue> {
        let (name, attrs, styles, content, listeners) = self.resources();
        let ele = doc.create_element(name).map_err(|e| {
            tracing::error!("failed to create element {}: {:?}", name, e);
            e
        })?;
        for (name, value) in attrs.0.iter() {
            ele.set_attribute(name, value).map_err(|e| {
                tracing::error!("failed to set attribute {}=\"{}\", {:?}", name, value, e);
                e
            })?;
        }

        if !styles.0.is_empty() {
            let styles = styles
                .0
                .iter()
                .map(|(k, v)| format!("{k}:{v};"))
                .collect::<Vec<_>>()
                .join("");
            ele.set_attribute("style", &styles).map_err(|e| {
                tracing::error!("failed to set style on {}: {:?}", name, e);
                e
            })?;
        }

        for (kind, factory) in listeners.into_iter() {
            let cb = Closure::wrap(factory());
            ele.add_event_listener_with_callback(kind, cb.as_ref().unchecked_ref())
                .map_err(|e| {
                    tracing::error!(
                        "failed to bind {} event listener on {}: {:?}",
                        kind,
                        name,
                        e
                    );
                    e
                })?;
            cb.forget()
        }

        self.set_element(ele.clone());
        parent.append_child(&ele).map_err(|e| {
            tracing::error!(
                "failed to append children {} to parent {}: {:?}",
                name,
                parent.local_name(),
                e
            );
            e
        })?;
        match content {
            Content::Null => {}
            Content::Text(text) => {
                ele.set_inner_html(text);
            }
            Content::List(children) => {
                for child in children {
                    let child = child.render(&ele, doc)?;
                    ele.append_child(&child)?;
                }
            }
        };
        Ok(ele)
    }

    fn format(&self, f: &mut TagFormatter, buf: &mut String) -> std::fmt::Result {
        use std::fmt::Write;

        let pad = f.pad_size();
        let (name, attrs, styles, content, _) = self.resources();
        if name == "html" {
            write!(buf, "<!doctype html>")?;
        }
        write!(buf, "{:pad$}<{}", "", name)?;
        if f.newline_on_attr {
            buf.push_str(f.line_sep);
            let pad = pad + 1;
            write!(buf, "{:pad$}", "")?;
            for (name, val) in attrs.0.iter() {
                if val.is_empty() {
                    write!(buf, r#"{:pad$}{}"#, "", name)?;
                } else {
                    write!(buf, r#"{:pad$}{}="{}""#, "", name, val)?;
                }
                buf.push_str(f.line_sep);
            }
            if !styles.0.is_empty() {
                write!(buf, "{:pad$}", "")?;
                write!(buf, "style=\"")?;
                for (name, val) in styles.0.iter() {
                    write!(buf, "{}: {}; ", name, val)?;
                }
                write!(buf, "\"")?;
            }
            let pad = pad - 1;
            write!(buf, "{:pad$}>", "")?;
            buf.push_str(f.line_sep);
        } else {
            for (name, val) in attrs.0.iter() {
                if val.is_empty() {
                    write!(buf, r#" {}"#, name)?;
                } else {
                    write!(buf, r#" {}="{}""#, name, val)?;
                }
            }
            if !styles.0.is_empty() {
                write!(buf, " style=\"")?;
                for (name, val) in styles.0.iter() {
                    write!(buf, "{}: {}; ", name, val)?;
                }
                write!(buf, "\"")?;
            }
            buf.push('>');
        }
        match content {
            Content::Null => {
                write!(buf, "</{name}>")?;
            }
            Content::Text(text) => {
                buf.write_str(text)?;
                write!(buf, "</{name}>")?;
            }
            Content::List(children) => {
                buf.push_str(f.line_sep);
                f.indent += 1;
                for child in children.iter() {
                    child.format(f, buf)?
                }
                f.indent -= 1;
                if f.newline_on_attr {
                    buf.push_str(f.line_sep);
                    let pad = f.pad_size();
                    write!(buf, "{:pad$}</{name}>", "")?;
                } else {
                    write!(buf, "</{name}>")?;
                }
            }
        }
        buf.push_str(f.line_sep);
        Ok(())
    }
}

/// a trait represent multi markers
pub trait Markers {
    fn set_this(&self, element: Element);
}

#[derive(Debug, Clone)]
pub struct Marker<D = ()> {
    pub ele: Rc<RefCell<Option<Element>>>,
    pub data: Rc<RefCell<D>>,
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            ele: Default::default(),
            data: Rc::new(RefCell::new(())),
        }
    }
}

impl<D> Marker<D> {
    pub fn is_init(&self) -> bool {
        self.ele.borrow().is_some()
    }

    pub fn bind(data: Rc<RefCell<D>>) -> Marker<D> {
        Self {
            data,
            ele: Rc::new(RefCell::new(None)),
        }
    }

    pub fn inject(data: D) -> Marker<D> {
        Self {
            data: Rc::new(RefCell::new(data)),
            ele: Rc::new(RefCell::new(None)),
        }
    }

    pub fn set(&self, ele: Element) -> Option<Element> {
        self.ele.borrow_mut().replace(ele)
    }

    pub fn to<T>(&self, data: Rc<RefCell<T>>) -> Marker<T> {
        let ele = self.ele.clone();
        Marker { ele, data }
    }

    pub fn get_ele(&self) -> Element {
        self.ele.borrow().as_ref().cloned().unwrap()
    }
}

pub fn tag(name: &'static str, content: Content) -> tags::Unit<(Marker,)> {
    tags::Unit {
        name,
        content,
        styles: Default::default(),
        attrs: Default::default(),
        markers: (Marker::default(),),
        listeners: Default::default(),
    }
}

/// extend one marker with a tuple of marker
///
/// ```bash
/// Marker<A> + Marker<B> -> (Marker<A>, Marker<B>)
///
/// Marker<A> + (Marker<B>, Marker<C>) -> (Marker<A>, Marker<B>, Marker<C>)
/// ```
pub trait ExtendMarkers<Rhs = Self> {
    type Output: Markers;

    fn extend(self, rhs: Rhs) -> Self::Output;
}

/// default entry point of app, mount top most element
/// to document body directory.
pub fn mount_body<T: Template>(tag: T) -> Result<(Window, Document, HtmlElement), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    if body.child_element_count() != 0 {
        tracing::warn!("body children is not empty");
    }
    tag.render(&body, &document)?;
    Ok((window, document, body))
}
