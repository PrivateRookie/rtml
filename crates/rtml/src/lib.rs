use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};
use tag_fmt::TagFormatter;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, Event, HtmlElement};

mod basic_impl;
mod events;
/// html tags
pub mod tags;
pub use events::*;
use tags::{Attrs, StaticAttrs, StaticStyles, Styles};
mod attr;
mod reactive;
pub use reactive::{reactive, CombinedReactive, IntoReactive, Reactive};
pub use rtml_macro::page;
/// print template as html, instead of create them by dom api
pub mod tag_fmt;

fn get_ele_pos(ele: &Element) -> (usize, usize) {
    let mut depth = 0;
    let mut parent = ele.parent_element();
    loop {
        if let Some(par) = parent {
            depth += 1;
            parent = par.parent_element();
        } else {
            break;
        }
    }
    let mut offset = 0;
    let mut prev = ele.previous_element_sibling();
    loop {
        if let Some(pre) = prev {
            offset += 1;
            prev = pre.previous_element_sibling();
        } else {
            break;
        }
    }
    (depth, offset)
}

pub(crate) fn render_children(
    children: &Children,
    ele: &Element,
    doc: &Document,
) -> Result<(), JsValue> {
    match children {
        Children::Null => {}
        Children::Text(text) => {
            ele.set_inner_html(text);
        }
        Children::List(children) => {
            for child in children {
                let child = child.render(ele, doc)?;
                ele.append_child(&child)?;
            }
        }
        Children::Dynamic(view) => {
            let ele_pos = get_ele_pos(ele);
            match view {
                ViewToken::Owned { view, subs } => {
                    let subs = subs.clone();
                    let view = view.replace(Box::new(|| Children::Null));
                    let content = view();
                    let pos = subs
                        .borrow()
                        .binary_search_by(|item| item.0.cmp(&ele_pos))
                        .unwrap_or_else(|e| e);
                    subs.borrow_mut()
                        .insert(pos, (ele_pos, ele.clone(), ViewFunc::Owned(view)));
                    render_children(&content, ele, doc)?;
                }
                ViewToken::Shared { view, subs } => {
                    let content = view.borrow()();
                    let ele_pos = get_ele_pos(ele);
                    for item in subs.iter() {
                        let pos = item
                            .borrow()
                            .binary_search_by(|item| item.0.cmp(&ele_pos))
                            .unwrap_or_else(|e| e);
                        item.borrow_mut()
                            .insert(pos, (ele_pos, ele.clone(), ViewFunc::Shared(view.clone())));
                    }
                    render_children(&content, ele, doc)?;
                }
            }
        }
    };
    Ok(())
}

pub type TplResources<'a> = (
    &'static str,
    &'a Attrs,
    &'a Styles,
    &'a Children,
    HashMap<&'a str, Box<dyn Fn(Event)>>,
);

pub trait Template {
    /// provide tag name, content, attr, styles and listeners
    /// to create html element
    fn resources(&self) -> TplResources;

    /// generate html element and add event bindings
    fn render(&self, parent: &Element, doc: &Document) -> Result<Element, JsValue> {
        let (name, attrs, styles, content, other_listeners) = self.resources();
        let ele = doc.create_element(name).map_err(|e| {
            tracing::error!("failed to create element {}: {:?}", name, e);
            e
        })?;

        let attrs = attrs.val();
        for (name, value) in attrs.iter() {
            ele.set_attribute(name, value).map_err(|e| {
                tracing::error!("failed to set attribute {}=\"{}\", {:?}", name, value, e);
                e
            })?;
        }

        styles.config_element(&ele)?;

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
        render_children(content, &ele, doc)?;
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
            Children::Null => {
                write!(buf, "</{name}>")?;
            }
            Children::Text(text) => {
                buf.write_str(text)?;
                write!(buf, "</{name}>")?;
            }
            Children::List(children) => {
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
            Children::Dynamic(_) => {
                write!(buf, "dynamic content")?;
            }
        }
        buf.push_str(f.line_sep);
        Ok(())
    }
}

pub fn unit(name: &'static str, content: Children) -> tags::Unit {
    tags::Unit {
        name,
        content,
        styles: Default::default(),
        attrs: Default::default(),
        listeners: Default::default(),
    }
}

/// html element content
pub enum Children {
    /// empty tag
    Null,
    /// this element has children
    List(TemplateList),
    // TODO add html escape checking
    /// normal text content
    Text(String),
    Dynamic(ViewToken),
}

impl Default for Children {
    fn default() -> Self {
        Self::Null
    }
}

type ChildrenFunc = dyn Fn() -> Children;

pub type ViewSubs = Rc<RefCell<Vec<((usize, usize), Element, ViewFunc)>>>;
pub type AttrSubs = Rc<RefCell<Vec<(Element, AttrFunc)>>>;
pub type StyleSubs = Rc<RefCell<Vec<(Element, StyleFunc)>>>;

pub enum ViewFunc {
    Owned(Box<ChildrenFunc>),
    Shared(Rc<RefCell<ChildrenFunc>>),
}

impl ViewFunc {
    pub fn get_children(&self) -> Children {
        match self {
            ViewFunc::Owned(func) => func(),
            ViewFunc::Shared(func) => func.borrow()(),
        }
    }
}

pub enum ViewToken {
    Owned {
        view: Cell<Box<ChildrenFunc>>,
        subs: ViewSubs,
    },
    Shared {
        view: Rc<RefCell<ChildrenFunc>>,
        subs: Vec<ViewSubs>,
    },
}

pub enum AttrToken {
    Owned {
        view: RefCell<Box<dyn Fn() -> StaticAttrs>>,
        subs: AttrSubs,
    },
    Shared {
        view: Rc<RefCell<dyn Fn() -> StaticAttrs>>,
        subs: Vec<AttrSubs>,
    },
}

impl AttrToken {
    pub fn val(&self) -> StaticAttrs {
        match self {
            AttrToken::Owned { view, .. } => view.borrow()(),
            AttrToken::Shared { view, .. } => view.borrow()(),
        }
    }
}

pub enum AttrFunc {
    Owned(Box<dyn Fn() -> StaticAttrs>),
    Shared(Rc<RefCell<dyn Fn() -> StaticAttrs>>),
}

impl AttrFunc {
    pub fn val(&self) -> StaticAttrs {
        match self {
            AttrFunc::Owned(func) => func(),
            AttrFunc::Shared(func) => func.borrow()(),
        }
    }
}

pub enum StyleToken {
    Owned {
        view: RefCell<Box<dyn Fn() -> StaticStyles>>,
        subs: StyleSubs,
    },
    Shared {
        view: Rc<RefCell<dyn Fn() -> StaticStyles>>,
        subs: Vec<StyleSubs>,
    },
}

impl StyleToken {
    pub fn val(&self) -> StaticStyles {
        match self {
            StyleToken::Owned { view, .. } => view.borrow()(),
            StyleToken::Shared { view, .. } => view.borrow()(),
        }
    }
}

pub enum StyleFunc {
    Owned(Box<dyn Fn() -> StaticStyles>),
    Shared(Rc<RefCell<dyn Fn() -> StaticStyles>>),
}

impl StyleFunc {
    pub fn val(&self) -> StaticStyles {
        match self {
            StyleFunc::Owned(func) => func(),
            StyleFunc::Shared(func) => func.borrow()(),
        }
    }
}

pub type TemplateList = Vec<Box<dyn Template>>;
pub type Listeners = HashMap<&'static str, Box<dyn Fn() -> Box<dyn Fn(Event)>>>;

fn get_document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}

/// default entry point of app, mount top most element
/// to document body directory.
pub fn mount_body<C: Into<Children>>(children: C) -> Result<(Document, HtmlElement), JsValue> {
    let document = get_document();
    let body = document.body().expect("document should have a body");

    if body.child_element_count() != 0 {
        tracing::warn!("body children is not empty");
    }
    let children = children.into();
    render_children(&children, &body, &document)?;
    Ok((document, body))
}

pub fn mount<C: Into<Children>>(target: &Element, children: C) -> Result<(), JsValue> {
    let doc = get_document();
    let children = children.into();
    render_children(&children, target, &doc)?;
    Ok(())
}
