use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use wasm_bindgen::JsValue;
use web_sys::Element;

use crate::{render_content, tag_fmt::TagFormatter, Template};

mod builtin;
pub use builtin::*;

/// html element content
pub enum Content {
    /// empty tag
    Null,
    /// this element has children
    List(TemplateList),
    // TODO add html escape checking
    /// normal text content
    Text(String),
    Dynamic(ViewCredential),
}

impl Default for Content {
    fn default() -> Self {
        Self::Null
    }
}

pub type TemplateList = Vec<Box<dyn Template>>;
pub type Listeners = HashMap<&'static str, Box<dyn Fn() -> Box<dyn Fn(JsValue)>>>;

pub struct Unit {
    pub name: &'static str,
    pub content: Content,
    pub attrs: Attrs,
    pub styles: Styles,
    pub listeners: Listeners,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut content = String::new();
        let mut formatter = TagFormatter::default();
        self.format(&mut formatter, &mut content)?;
        write!(f, "{}", content)
    }
}

impl Template for Unit {
    fn resources(
        &self,
    ) -> (
        &'static str,
        &Attrs,
        &Styles,
        &Content,
        HashMap<&str, Box<dyn Fn(JsValue)>>,
    ) {
        let other_factories: HashMap<&'static str, _> = self
            .listeners
            .iter()
            .map(|(kind, func)| {
                let cb = func();
                (*kind, cb)
            })
            .collect();
        (
            self.name,
            &self.attrs,
            &self.styles,
            &self.content,
            other_factories,
        )
    }
}

impl Unit {
    pub fn on<K: Into<&'static str>>(
        mut self,
        kind: K,
        listener: Box<dyn Fn() -> Box<dyn Fn(JsValue)>>,
    ) -> Self {
        self.listeners.insert(kind.into(), listener);
        self
    }
}

/// simple wrapper of tag style
#[derive(Debug, Clone, Default)]
pub struct Styles(pub HashMap<String, String>);

#[doc(hidden)]
pub fn _warning(msg: &str) {
    tracing::warn!(msg)
}

/// simple wrapper of tag attributes
#[derive(Debug, Clone, Default)]
pub struct Attrs(pub HashMap<String, String>);

/// helper macro to set html element attributes, like id, class, hidden etc
///
/// ```no_run
/// div((
///     attr! { id="app", data-some-demo },
///     "hello"
///  ))
/// ```
///
/// create such a html element
///
/// ```html
/// <div id="app" data-some-demo>hello</div>
/// ```
///
/// **NOTE**: style should be set by `style!` macro
#[macro_export]
macro_rules! attr {
    ($($($name:tt)-+ $(= $value:expr)?),+ $(,)?) => {
        #[allow(unused_assignments)]
        { let mut attrs: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        $(
            let name = vec![$(stringify!($name)),*];
            let key = name.join("-");
            let mut valid = true;
            if key.starts_with("on") {
                $crate::tags::_warning("event handler can not be set with attr! macro");
                valid = false;
            } else if key == "style" {
                $crate::tags::_warning("style should be set with style! macro");
                valid = false;
            }
            let mut value = None;
            $(
                value = Some($value.to_string());
            )?
            if valid {
                attrs.insert(key.to_string(), value.unwrap_or_default());
            }

        )*
        $crate::tags::Attrs(attrs)
    }};
    () => {{
        $crate::tags::Attrs::default()
    }}
}

pub type Subs = Rc<RefCell<Vec<(Element, Box<dyn Fn() -> Content>)>>>;

#[derive(Clone)]
pub enum ChangeSubscribers {
    Single(Subs),
    Combine(Vec<Subs>),
}

pub struct ViewCredential {
    pub(crate) view: Cell<Box<dyn Fn() -> Content>>,
    pub(crate) subs: Subs,
}

impl ViewCredential {
    pub fn new(subs: Subs, view: Box<dyn Fn() -> Content + 'static>) -> Self {
        Self {
            view: Cell::new(view),
            subs,
        }
    }
}

pub struct Reactive<T> {
    pub data: Rc<RefCell<T>>,
    pub subscribers: Subs,
}

impl<T> Clone for Reactive<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            subscribers: self.subscribers.clone(),
        }
    }
}

pub fn reactive<T>(data: T) -> Reactive<T> {
    Reactive {
        data: Rc::new(RefCell::new(data)),
        subscribers: Rc::new(RefCell::new(vec![])),
    }
}

pub trait IntoReactive {
    fn reactive(self) -> Reactive<Self>
    where
        Self: Sized;
}

impl<T: Sized> IntoReactive for T {
    fn reactive(self) -> Reactive<Self>
    where
        Self: Sized,
    {
        reactive(self)
    }
}

impl<T: 'static> Reactive<T> {
    /// get inner data ref
    pub fn val(&self) -> Ref<T> {
        self.data.borrow()
    }

    /// get inner data mut ref
    pub fn val_mut(&self) -> RefMut<T> {
        self.data.borrow_mut()
    }

    pub fn view<V: Fn(Self) -> Content + 'static>(&self, v: V) -> ViewCredential {
        let data = self.clone();
        ViewCredential::new(self.subscribers.clone(), Box::new(move || v(data.clone())))
    }

    /// handle event and reactive data
    pub fn evt<M: Fn(JsValue, Self) + 'static + Copy>(
        &self,
        m: M,
    ) -> Box<dyn Fn() -> Box<dyn Fn(JsValue)> + 'static> {
        let data = self.clone();
        Box::new(move || {
            let data = data.clone();
            Box::new(move |event| m(event, data.clone()))
        })
    }

    /// mutate data and update view
    pub fn change<M: Fn(Self) + 'static + Copy>(
        &self,
        m: M,
    ) -> Box<dyn Fn() -> Box<dyn Fn(JsValue)> + 'static> {
        let rea = self.clone();
        Box::new(move || {
            let rea = rea.clone();
            Box::new(move |_| {
                m(rea.clone());
                update_and_clear(&rea);
            })
        })
    }

    pub fn update(&self) {
        update_and_clear(self);
    }
}

fn update_and_clear<T>(rea: &Reactive<T>) {
    for (ele, view) in rea.subscribers.borrow().iter() {
        let children = ele.children();
        for _ in 0..children.length() {
            if let Some(child) = children.get_with_index(0) {
                ele.remove_child(&child).unwrap();
            }
        }
        let content = view();
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        if let Err(e) = render_content(&content, ele, &document) {
            tracing::error!("failed to update content: {:?}", e);
        }
    }
    // remove not active elements
    rea.subscribers
        .borrow_mut()
        .sort_by(|a, b| (!a.0.is_connected()).cmp(&(!b.0.is_connected())));
    let pos = rea
        .subscribers
        .borrow()
        .iter()
        .position(|i| !i.0.is_connected());
    if let Some(pos) = pos {
        rea.subscribers.borrow_mut().drain(pos..);
    }
}

//Reactive<T> impl Display ,Print data status
/// let any_data = any_data.reactive();
/// let data = format!("{}",any_data);
/// console.log(data);
impl<T: 'static + std::fmt::Debug> std::fmt::Display for Reactive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.borrow();
        write!(f, "{:?}", *data)
    }
}
// !!!!
impl<T> From<T> for Reactive<T> {
    fn from(t:T) -> Reactive<T> {
        reactive(t)
    }
}

// !!!!
impl<T> Into<Rc<RefCell<T>>> for Reactive<T> {
    fn into(self) -> Rc<RefCell<T>> {
        self.data
    }
}

pub struct CombinedReactive<T> {
    data: T,
}

impl<T: Clone> Clone for CombinedReactive<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<A, B> std::ops::Add<Reactive<B>> for Reactive<A> {
    type Output = CombinedReactive<(Reactive<A>, Reactive<B>)>;

    fn add(self, rhs: Reactive<B>) -> Self::Output {
        CombinedReactive { data: (self, rhs) }
    }
}

macro_rules! impl_combine {
    ($($t:tt),+ | $($i:tt),+) => {
        impl<This, $($t ),+> std::ops::Add<($($crate::tags::Reactive<$t>),+,)> for $crate::tags::Reactive<This> {
            type Output = $crate::tags::CombinedReactive<($crate::tags::Reactive<This>, $($crate::tags::Reactive<$t>),+)>;

            fn add(self, rhs: ($($crate::tags::Reactive<$t>),+,)) -> Self::Output {
                $crate::tags::CombinedReactive {
                    data: (self.clone(), $(rhs.$i),+)
                }
            }
        }


        impl<This: 'static, $($t: 'static),+> $crate::tags::CombinedReactive<($crate::tags::Reactive<This>, $($crate::tags::Reactive<$t>),+)> {
            /// mutate data and update view
            pub fn change<Method: Fn(($crate::tags::Reactive<This>, $($crate::tags::Reactive<$t>),+)) + 'static + Copy>(
                &self,
                m: Method,
            ) -> Box<dyn Fn() -> Box<dyn Fn(wasm_bindgen::JsValue)> + 'static>  {
                let data = self.data.clone();
                Box::new(move|| {
                    let data = data.clone();
                    Box::new(move |_| {
                        m(data.clone());
                        $(
                            data.$i.update();
                        )+
                    })
                })
            }

            /// mutate data and update view
            pub fn evt<Method: Fn(wasm_bindgen::JsValue, ($crate::tags::Reactive<This>, $($crate::tags::Reactive<$t>),+)) + 'static + Copy>(
                &self,
                m: Method,
            ) -> Box<dyn Fn() -> Box<dyn Fn(wasm_bindgen::JsValue)> + 'static>  {
                let data = self.data.clone();
                Box::new(move|| {
                    let data = data.clone();
                    Box::new(move |event| {
                        m(event, data.clone());
                        $(
                            data.$i.update();
                        )+
                    })
                })
            }
        }
    };
}

#[rustfmt::skip]
mod inner {
    impl_combine!( A  |  0 );
    impl_combine!( A, B  |  0, 1 );
    impl_combine!( A, B, C  |  0, 1, 2 );
    impl_combine!( A, B, C, D  |  0, 1, 2, 3 );
    impl_combine!( A, B, C, D, E  |  0, 1, 2, 3, 4 );
    impl_combine!( A, B, C, D, E, F  |  0, 1, 2, 3, 4, 5 );
    impl_combine!( A, B, C, D, E, F, G  |  0, 1, 2, 3, 4, 5, 6 );
    impl_combine!( A, B, C, D, E, F, G, H  |  0, 1, 2, 3, 4, 5, 6, 7 );
    impl_combine!( A, B, C, D, E, F, G, H, I  |  0, 1, 2, 3, 4, 5, 6, 7, 8 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 );
}
