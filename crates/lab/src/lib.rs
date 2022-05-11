use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element};

pub trait Template {
    fn take(
        &self,
    ) -> (
        &'static str,
        &Content,
        HashMap<EventKind, Box<dyn FnOnce() -> Box<dyn FnMut()> + '_>>,
    );

    fn set_element(&self, element: Element);

    fn render(&self, parent: &Element, doc: &Document) -> Result<Element, JsValue> {
        let (name, content, listeners) = self.take();
        let ele = match content {
            Content::Null => parent.clone(),
            Content::Text(text) => {
                let ele = doc.create_element(name)?;
                self.set_element(ele.clone());
                ele.set_inner_html(&text);
                parent.append_child(&ele)?;
                for (kind, factory) in listeners.into_iter() {
                    let cb = Closure::wrap(factory());
                    ele.add_event_listener_with_callback(
                        kind.as_ref(),
                        cb.as_ref().unchecked_ref(),
                    )?;
                    cb.forget()
                }
                ele
            }
            Content::List(children) => {
                let ele = doc.create_element(name)?;
                self.set_element(ele.clone());
                for (kind, factory) in listeners.into_iter() {
                    let cb = Closure::wrap(factory());
                    ele.add_event_listener_with_callback(
                        kind.as_ref(),
                        cb.as_ref().unchecked_ref(),
                    )?;
                    cb.forget()
                }
                for child in children {
                    let child = child.render(&ele, doc)?;
                    ele.append_child(&child)?;
                }
                parent.append_child(&ele)?;
                ele
            }
        };
        Ok(ele)
    }
}

pub type TemplateList = Vec<Box<dyn Template>>;

pub enum Content {
    Null,
    List(TemplateList),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventKind {
    Click,
    Abort,
}

impl AsRef<str> for EventKind {
    fn as_ref(&self) -> &str {
        match self {
            EventKind::Click => "click",
            EventKind::Abort => "abort",
        }
    }
}

pub type Listener = Box<dyn Fn() -> Box<dyn FnMut()>>;

pub trait Markers {
    fn set_this(&self, element: Element);
}

pub struct Unit<M: Markers> {
    pub name: &'static str,
    pub content: Content,
    pub markers: M,
    pub listeners: HashMap<EventKind, Box<dyn Fn(M) -> Box<dyn FnMut()>>>,
}

impl<M: Markers + Clone> Template for Unit<M> {
    fn take(
        &self,
    ) -> (
        &'static str,
        &Content,
        HashMap<EventKind, Box<dyn FnOnce() -> Box<dyn FnMut()> + '_>>,
    ) {
        let factories: HashMap<EventKind, _> = self
            .listeners
            .iter()
            .map(|(kind, func)| {
                let cb = Box::new(move || {
                    func(self.markers.clone())
                }) as Box<dyn FnOnce() -> Box<dyn FnMut()>>;
                let kind = kind.clone();
                (kind, cb)
            })
            .collect();
        (self.name, &self.content, factories)
    }

    fn set_element(&self, element: Element) {
        self.markers.set_this(element);
    }
}

impl<D: Clone> Unit<(Marker<D>,)> {
    pub fn link<R>(self, other: R) -> Unit<<(Marker<D>,) as Merge<R>>::Output>
    where
        (Marker<D>,): Merge<R>,
    {
        let Self {
            name,
            content,
            markers,
            listeners: _,
        } = self;
        Unit {
            name,
            content,
            markers: markers.merge(other),
            listeners: Default::default(),
        }
    }
}

impl Unit<(Marker,)> {
    pub fn with<D: Clone>(self, data: Rc<RefCell<D>>) -> Unit<(Marker<D>,)> {
        let Self {
            name,
            content,
            markers,
            listeners: _,
        } = self;
        Unit {
            name,
            content,
            markers: (markers.0.to(data),),
            listeners: Default::default(),
        }
    }
}

impl<M: Markers + Clone> Unit<M> {
    pub fn mark(&self) -> M {
        self.markers.clone()
    }

    pub fn on(mut self, kind: EventKind, listener: Box<dyn Fn(M) -> Box<dyn FnMut()>>) -> Self {
        self.listeners.insert(kind, listener);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Marker<D = ()> {
    pub(crate) ele: Rc<RefCell<Option<Element>>>,
    pub data: Rc<RefCell<D>>,
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

pub fn tag(name: &'static str, content: Content) -> Unit<(Marker,)> {
    Unit {
        name,
        content,
        markers: (mark(()),),
        listeners: Default::default(),
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let p = tag("p", Content::Text("no data yet".to_string()));
    let btn = tag("button", Content::Text("click".to_string()))
        .with(Rc::new(RefCell::new(0usize)))
        .link(p.mark())
        .on(
            EventKind::Click,
            Box::new(|(btn, show)| {
                Box::new(move || {
                    tracing::info!("clicked");
                    let mut count = btn.data.borrow_mut();
                    *count += 1;
                    let e = show.ele.borrow();
                    let e = e.as_ref().unwrap();
                    e.set_inner_html(&count.to_string());
                })
            }),
        );
    let p = Box::new(p);
    let btn = Box::new(btn);
    let container = tag("div", Content::List(vec![p, btn]));
    container.render(&body, &document).unwrap();
}

fn mark<D>(data: D) -> Marker<D> {
    Marker {
        ele: Default::default(),
        data: Rc::new(RefCell::new(data)),
    }
}

pub trait Merge<Rhs = Self> {
    type Output: Markers;

    fn merge(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_collection {
    ($($ty:tt),+) => {
        impl<$($ty: Clone),+> $crate::Markers  for ($($crate::Marker<$ty>),+ ,) {
            fn set_this(&self, element: Element) {
                self.0.set(element);
            }
        }
    };
}

impl_collection!(A);
impl_collection!(A, B);
impl_collection!(A, B, C);
impl_collection!(A, B, C, D);
impl_collection!(A, B, C, D, E);
impl_collection!(A, B, C, D, E, F);
impl_collection!(A, B, C, D, E, F, G);
impl_collection!(A, B, C, D, E, F, G, H);
impl_collection!(A, B, C, D, E, F, G, H, I);
impl_collection!(A, B, C, D, E, F, G, H, I, J);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
impl_collection!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA);

#[rustfmt::skip]
mod merge_impl {
    macro_rules! impl_merge {
        ($s:tt | $($ty:tt),+ | $($i:tt),+) => {
            impl <$s: Clone, $($ty: Clone),+> $crate::Merge<($($crate::Marker<$ty>),+ ,)> for ($crate::Marker<$s>,) {
                type Output = ($crate::Marker<$s>, $($crate::Marker<$ty>),+);
    
                fn merge(self, rhs: ($($crate::Marker<$ty>),+ ,)) -> Self::Output {
                    (self.0, $(rhs.$i),+)
                }
            }
        }
    }

    impl_merge!(Init | A | 0);
    impl_merge!(Init | A, B | 0, 1);
    impl_merge!(Init | A, B, C | 0, 1, 2);
    impl_merge!(Init | A, B, C, D | 0, 1, 2, 3);
    impl_merge!(Init | A, B, C, D, E | 0, 1, 2, 3, 4);
    impl_merge!(Init | A, B, C, D, E, F | 0, 1, 2, 3, 4, 5);
    impl_merge!(Init | A, B, C, D, E, F, G | 0, 1, 2, 3, 4, 5, 6);
    impl_merge!(Init | A, B, C, D, E, F, G, H | 0, 1, 2, 3, 4, 5, 6, 7);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I | 0, 1, 2, 3, 4, 5, 6, 7, 8);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);
}
