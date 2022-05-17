use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

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

pub struct Unit {
    pub name: &'static str,
    pub content: Content,
    pub attrs: Attrs,
    pub styles: Styles,
    pub other_listeners: HashMap<&'static str, Box<dyn Fn() -> Box<dyn FnMut()>>>,
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
        HashMap<&str, Box<dyn FnMut()>>,
    ) {
        let other_factories: HashMap<&'static str, _> = self
            .other_listeners
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
        listener: Box<dyn Fn() -> Box<dyn FnMut()>>,
    ) -> Self {
        self.other_listeners.insert(kind.into(), listener);
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
    ($($($name:tt)-+ $(= $value:expr)?),+) => {
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
#[derive(Clone)]
pub struct Reactive<T> {
    pub data: Rc<RefCell<T>>,
    pub subscribers: Subs,
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
    pub fn view<V: Fn(&T) -> Content + 'static>(&self, v: V) -> ViewCredential {
        let data = self.data.clone();
        ViewCredential::new(
            self.subscribers.clone(),
            Box::new(move || v(&data.borrow())),
        )
    }

    pub fn mutate<M: FnMut(&mut T) + 'static + Copy>(
        &self,
        mut m: M,
    ) -> Box<dyn Fn() -> Box<dyn FnMut()> + 'static> {
        let data = self.data.clone();
        let subs = self.subscribers.clone();
        Box::new(move || {
            let data = data.clone();
            let subs = subs.clone();
            Box::new(move || {
                m(&mut data.borrow_mut());
                for (ele, view) in subs.borrow().iter() {
                    if ele.is_connected() {
                        let content = view();
                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");
                        if let Err(e) = render_content(&content, &ele, &document) {
                            tracing::error!("failed to update content: {:?}", e);
                        }
                    }
                }
            })
        })
    }
}
