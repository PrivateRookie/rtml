use crate::{tag_fmt::TagFormatter, Listeners, Template};
use std::collections::HashMap;
use wasm_bindgen::JsValue;

mod builtin;
pub use builtin::*;

pub struct Unit {
    pub name: &'static str,
    pub content: crate::Children,
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
        &crate::Children,
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
