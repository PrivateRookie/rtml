use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element};

mod basic_impl;
mod events;
pub mod tags;
pub use events::*;
use tags::Content;
mod attr;
mod style;

pub trait Template {
    /// provide tag name, content, attr, styles and listeners
    /// to create html element
    fn resources(
        &self,
    ) -> (
        &'static str,
        &Content,
        HashMap<&str, Box<dyn FnOnce() -> Box<dyn FnMut()> + '_>>,
    );

    /// after calling dom api to create element,
    /// replace marker ele field with real element
    fn set_element(&self, element: Element);

    /// generate html element and add event bindings
    fn render(&self, parent: &Element, doc: &Document) -> Result<Element, JsValue> {
        let (name, content, listeners) = self.resources();
        let ele = doc.create_element(name)?;
        self.set_element(ele.clone());
        parent.append_child(&ele)?;
        for (kind, factory) in listeners.into_iter() {
            let cb = Closure::wrap(factory());
            ele.add_event_listener_with_callback(kind, cb.as_ref().unchecked_ref())?;
            cb.forget()
        }
        match content {
            Content::Null => {}
            Content::Text(text) => {
                ele.set_inner_html(&text);
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
}

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

    pub fn set(&self, ele: Element) -> Option<Element> {
        self.ele.borrow_mut().replace(ele)
    }

    pub fn to<T>(&self, data: Rc<RefCell<T>>) -> Marker<T> {
        let ele = self.ele.clone();
        Marker { ele, data }
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

pub trait Merge<Rhs = Self> {
    type Output: Markers;

    fn merge(self, rhs: Rhs) -> Self::Output;
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let p = tag("p", Content::Text("no data yet".to_string()));
    let btn = tag("button", Content::Text("click".to_string()))
        .take(0usize)
        .link(p.mark())
        .on(EventKind::Click, |(btn, show)| {
            Box::new(move || {
                tracing::info!("clicked");
                let mut count = btn.data.borrow_mut();
                *count += 1;
                let e = show.ele.borrow();
                let e = e.as_ref().unwrap();
                e.set_inner_html(&count.to_string());
            })
        });
    let p = Box::new(p);
    let btn = Box::new(btn);
    let container = tag("div", Content::List(vec![p, btn]));
    container.render(&body, &document).unwrap();
}
