use reactive::UpdateFunc;
use std::collections::HashMap;
use tag_fmt::TagFormatter;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, Event, HtmlElement};

mod basic_impl;
mod events;
/// html tags
pub mod tags;
pub use events::*;
use tags::{AttrRegisterData, Attrs, ContentRegisterData, EleContent, StyleRegisterData, Styles};
mod reactive;
pub use reactive::{reactive, CombinedReactive, IntoReactive, Reactive};
pub use rtml_macro::page;

use crate::tags::StaticContent;
/// print template as html, instead of create them by dom api
pub mod tag_fmt;

pub type TplResources<'a> = (
    &'static str,
    &'a Attrs,
    &'a Styles,
    &'a EleContent,
    HashMap<&'a str, Box<dyn Fn(Event)>>,
);

pub trait Template {
    /// provide tag name, content, attr, styles and listeners
    /// to create html element
    fn resources(&self) -> TplResources;

    /// generate html element and add event bindings
    fn render(
        &self,
        path: Vec<usize>,
        parent: &Element,
        doc: &Document,
    ) -> Result<Element, JsValue> {
        let (name, attrs, styles, content, other_listeners) = self.resources();
        let ele = doc.create_element(name).map_err(|e| {
            tracing::error!("failed to create element {}: {:?}", name, e);
            e
        })?;

        let attr_subs = attrs.config_element(&ele)?;
        let style_subs = styles.config_element(&ele)?;

        for (kind, factory) in other_listeners.into_iter() {
            let cb = Closure::wrap(factory);
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

        parent.append_child(&ele).map_err(|e| {
            tracing::error!(
                "failed to append children {} to parent {}: {:?}",
                name,
                parent.local_name(),
                e
            );
            e
        })?;
        let content_subs = content.render(path.clone(), &ele, doc)?;
        register(attr_subs, style_subs, content_subs, path, &ele);
        Ok(ele)
    }

    fn format(&self, f: &mut TagFormatter, buf: &mut String) -> std::fmt::Result {
        use std::fmt::Write;

        let pad = f.pad_size();
        let (name, attrs, styles, content, _) = self.resources();
        if name == "html" {
            write!(buf, "<!doctype html>{}", f.line_sep)?;
        }
        write!(buf, "{:pad$}<{}", "", name)?;
        if f.newline_on_attr {
            buf.push_str(f.line_sep);
            let pad = pad + 1;
            write!(buf, "{:pad$}", "")?;
            let attrs = attrs.val();
            for (name, val) in attrs.iter() {
                if val.is_empty() {
                    write!(buf, r#"{:pad$}{}"#, "", name)?;
                } else {
                    write!(buf, r#"{:pad$}{}="{}""#, "", name, val)?;
                }
                buf.push_str(f.line_sep);
            }
            let styles = styles.val();
            if !styles.is_empty() {
                write!(buf, "{:pad$}", "")?;
                write!(buf, "style=\"")?;
                for (name, val) in styles.iter() {
                    write!(buf, "{}: {}; ", name, val)?;
                }
                write!(buf, "\"")?;
            }
            let pad = pad - 1;
            write!(buf, "{:pad$}>", "")?;
            buf.push_str(f.line_sep);
        } else {
            let attrs = attrs.val();
            for (name, val) in attrs.iter() {
                if val.is_empty() {
                    write!(buf, r#" {}"#, name)?;
                } else {
                    write!(buf, r#" {}="{}""#, name, val)?;
                }
            }
            let styles = styles.val();
            if !styles.is_empty() {
                write!(buf, " style=\"")?;
                for (name, val) in styles.iter() {
                    write!(buf, "{}: {}; ", name, val)?;
                }
                write!(buf, "\"")?;
            }
            buf.push('>');
        }
        match content {
            EleContent::Static(content) => {
                fmt_static_content(name, content, buf, f)?;
            }
            EleContent::Dynamic { subs: _, func } => {
                let content = func.borrow()();
                fmt_static_content(name, &content, buf, f)?;
            }
        }
        buf.push_str(f.line_sep);
        Ok(())
    }
}

fn register(
    attr_subs: AttrRegisterData,
    style_subs: StyleRegisterData,
    content_subs: ContentRegisterData,
    path: Vec<usize>,
    ele: &Element,
) {
    match (attr_subs, style_subs, content_subs) {
        (None, None, None) => {}
        (None, None, Some((subs, func))) => {
            for dom in subs {
                dom.borrow_mut().register(
                    path.clone(),
                    ele.clone(),
                    UpdateFunc {
                        attr: None,
                        style: None,
                        children: Some(func.clone()),
                    },
                );
            }
        }
        (None, Some((subs, func)), None) => {
            for dom in subs {
                dom.borrow_mut().register(
                    path.clone(),
                    ele.clone(),
                    UpdateFunc {
                        attr: None,
                        style: Some(func.clone()),
                        children: None,
                    },
                );
            }
        }
        (None, Some((s_subs, s_func)), Some((c_subs, c_func))) => {
            assert_eq!(s_subs.len(), c_subs.len());
            for dom in s_subs {
                dom.borrow_mut().register(
                    path.clone(),
                    ele.clone(),
                    UpdateFunc {
                        attr: None,
                        style: Some(s_func.clone()),
                        children: Some(c_func.clone()),
                    },
                );
            }
        }
        (Some((subs, func)), None, None) => {
            for dom in subs {
                dom.borrow_mut().register(
                    path.clone(),
                    ele.clone(),
                    UpdateFunc {
                        attr: Some(func.clone()),
                        style: None,
                        children: None,
                    },
                );
            }
        }
        (Some((a_subs, a_func)), None, Some((c_subs, c_func))) => {
            assert_eq!(a_subs.len(), c_subs.len());
            for dom in a_subs {
                dom.borrow_mut().register(
                    path.clone(),
                    ele.clone(),
                    UpdateFunc {
                        attr: Some(a_func.clone()),
                        style: None,
                        children: Some(c_func.clone()),
                    },
                );
            }
        }
        (Some((a_subs, a_func)), Some((s_subs, s_func)), None) => {
            assert_eq!(a_subs.len(), s_subs.len());
            for dom in a_subs {
                dom.borrow_mut().register(
                    path.clone(),
                    ele.clone(),
                    UpdateFunc {
                        attr: Some(a_func.clone()),
                        style: Some(s_func.clone()),
                        children: None,
                    },
                );
            }
        }
        (Some((a_subs, a_func)), Some((s_subs, s_func)), Some((c_subs, c_func))) => {
            assert!(a_subs.len() == s_subs.len() && s_subs.len() == c_subs.len());
            for dom in a_subs {
                dom.borrow_mut().register(
                    path.clone(),
                    ele.clone(),
                    UpdateFunc {
                        attr: Some(a_func.clone()),
                        style: Some(s_func.clone()),
                        children: Some(c_func.clone()),
                    },
                );
            }
        }
    }
}

fn fmt_static_content(
    name: &str,
    content: &StaticContent,
    buf: &mut String,
    f: &mut TagFormatter,
) -> Result<(), std::fmt::Error> {
    use std::fmt::Write;
    match content {
        StaticContent::Null => {
            write!(buf, "</{name}>")?;
        }
        StaticContent::Text(text) => {
            buf.write_str(text)?;
            write!(buf, "</{name}>")?;
        }
        StaticContent::List(children) => {
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
    };
    Ok(())
}

pub fn unit(name: &'static str, content: EleContent) -> tags::Unit {
    tags::Unit {
        name,
        content,
        styles: Default::default(),
        attrs: Default::default(),
        listeners: Default::default(),
    }
}

pub type TemplateList = Vec<Box<dyn Template>>;
pub type Listeners = HashMap<&'static str, Box<dyn Fn() -> Box<dyn Fn(Event)>>>;

fn get_document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("should have a document on window")
}

fn get_element_offset(ele: &Element) -> usize {
    let mut offset = 0;
    let mut prev = ele.previous_element_sibling();
    while let Some(pre) = prev {
        offset += 1;
        prev = pre.previous_element_sibling();
    }
    offset
}

fn get_target_path(target: &Element) -> Vec<usize> {
    let mut reversed_path = vec![get_element_offset(target)];
    let mut parent = target.parent_element();
    while let Some(p) = parent {
        reversed_path.push(get_element_offset(&p));
        parent = p.parent_element();
    }
    reversed_path.reverse();
    reversed_path
}

/// default entry point of app, mount top most element
/// to document body directory.
pub fn mount_body<C: Into<EleContent>>(children: C) -> Result<(Document, HtmlElement), JsValue> {
    let document = get_document();
    let body = document.body().expect("document should have a body");

    if body.child_element_count() != 0 {
        tracing::warn!("body children is not empty");
    }
    let children = children.into();
    let path = get_target_path(&body);
    children.render(path, &body, &document)?;
    Ok((document, body))
}

pub fn mount<C: Into<EleContent>>(target: &Element, children: C) -> Result<(), JsValue> {
    let doc = get_document();
    let children = children.into();
    let path = get_target_path(target);
    children.render(path, target, &doc)?;
    Ok(())
}
