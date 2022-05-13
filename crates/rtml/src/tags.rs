use std::{cell::RefCell, collections::HashMap, rc::Rc};

use web_sys::Element;

use crate::{Marker, Markers, Merge, Template};

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
}

impl Default for Content {
    fn default() -> Self {
        Self::Null
    }
}

pub type TemplateList = Vec<Box<dyn Template>>;

pub struct Unit<M: Markers> {
    pub name: &'static str,
    pub content: Content,
    pub attrs: Attrs,
    pub styles: Styles,
    pub markers: M,
    pub listeners: HashMap<&'static str, Box<dyn Fn(M) -> Box<dyn FnMut()>>>,
}

impl<M: Markers + Clone> Template for Unit<M> {
    fn resources(
        &self,
    ) -> (
        &'static str,
        &Attrs,
        &Styles,
        &Content,
        HashMap<&str, Box<dyn FnOnce() -> Box<dyn FnMut()> + '_>>,
    ) {
        let factories: HashMap<&'static str, _> = self
            .listeners
            .iter()
            .map(|(kind, func)| {
                let cb = Box::new(move || func(self.markers.clone()))
                    as Box<dyn FnOnce() -> Box<dyn FnMut()>>;
                (*kind, cb)
            })
            .collect();
        (
            self.name,
            &self.attrs,
            &self.styles,
            &self.content,
            factories,
        )
    }

    fn set_element(&self, element: Element) {
        self.markers.set_this(element);
    }
}

impl<D: Clone> Unit<Marker<D>> {
    pub fn link<R>(self, other: R) -> Unit<<Marker<D> as Merge<R>>::Output>
    where
        Marker<D>: Merge<R>,
    {
        let Self {
            name,
            content,
            attrs,
            styles,
            markers,
            listeners: _,
        } = self;
        Unit {
            name,
            content,
            attrs,
            styles,
            markers: markers.merge(other),
            listeners: Default::default(),
        }
    }
}

impl Unit<Marker> {
    /// set associated element data
    pub fn bind<D>(self, data: Rc<RefCell<D>>) -> Unit<Marker<D>> {
        let Self {
            name,
            content,
            attrs,
            styles,
            markers,
            listeners: _,
        } = self;
        Unit {
            name,
            content,
            attrs,
            styles,
            markers: markers.to(data),
            listeners: Default::default(),
        }
    }

    /// set associated element data
    /// this method take the ownership of data
    pub fn inject<D>(self, data: D) -> Unit<Marker<D>> {
        let Self {
            name,
            content,
            attrs,
            styles,
            markers,
            listeners: _,
        } = self;
        Unit {
            name,
            content,
            attrs,
            styles,
            markers: markers.to(Rc::new(RefCell::new(data))),
            listeners: Default::default(),
        }
    }
}

impl<M: Markers + Clone> Unit<M> {
    pub fn mark(&self) -> M {
        self.markers.clone()
    }

    pub fn on<K: Into<&'static str>>(
        mut self,
        kind: K,
        listener: impl Fn(M) -> Box<dyn FnMut()> + 'static,
    ) -> Self {
        self.listeners.insert(kind.into(), Box::new(listener));
        self
    }
}

/// simple wrapper of tag style
#[derive(Debug, Clone, Default)]
pub struct Styles(pub HashMap<String, String>);

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
                tracing::warn!("event handler can not be set with attr! macro");
                valid = false;
            } else if key == "style" {
                tracing::warn!("style should be set with style! macro");
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
