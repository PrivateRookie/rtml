use crate::{tag_fmt::TagFormatter, Listeners, Template};
use std::collections::HashMap;

mod builtin;
pub use builtin::*;
use web_sys::Event;

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
        HashMap<&str, Box<dyn Fn(Event)>>,
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
        listener: Box<dyn Fn() -> Box<dyn Fn(Event)>>,
    ) -> Self {
        self.listeners.insert(kind.into(), listener);
        self
    }
}

#[derive(Debug, Default)]
pub struct StaticStyles(pub HashMap<String, String>);

pub(crate) fn config_style(
    styles: &StaticStyles,
    ele: &web_sys::Element,
) -> Result<(), wasm_bindgen::JsValue> {
    let styles = &styles.0;
    if !styles.is_empty() {
        let styles = styles
            .iter()
            .map(|(k, v)| format!("{k}:{v};"))
            .collect::<Vec<_>>()
            .join("");
        ele.set_attribute("style", &styles).map_err(|e| {
            tracing::error!("failed to set style on {}: {:?}", ele.local_name(), e);
            e
        })?;
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
mod style {
    use std::collections::HashMap;

    use wasm_bindgen::JsValue;
    use web_sys::Element;

    use super::{config_style, StaticStyles};

    pub type Styles = StaticStyles;

    impl Styles {
        pub fn val(&self) -> HashMap<String, String> {
            self.0.clone()
        }

        pub fn config_element(&self, ele: &Element) -> Result<(), JsValue> {
            config_style(&self, ele)
        }
    }

    // /// simple wrapper of tag style
    // #[derive(Debug, Clone, Default)]
    // pub struct Styles(pub HashMap<String, String>);

    /// helper macro to create css style
    ///
    /// ```no_run
    /// let s = style! {
    ///     background-color: "#fffff";
    ///     bar: "bxx";
    ///     font-size: 100;
    ///     escaped: r#""abc""#
    /// };
    /// ```
    /// generated
    /// ```css
    /// background-color: #fffff;
    /// bar: bxx;
    /// font-size: 100;
    /// escaped: "abc";
    /// ```
    #[macro_export]
    macro_rules! style {
        ($($($name:ident)-+: $value:expr);+ $(;)?) => {
            {
                let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                $(
                    let name = vec![$(stringify!($name)),*];
                    map.insert(name.join("-"), $value.to_string());
                )+
                $crate::tags::StaticStyles(map)
            }
        };
        () => {
            $crate::tags::StaticStyles::default()
        }
    }
}

#[cfg(target_family = "wasm")]
mod style {
    use crate::{StyleFunc, StyleToken};

    use super::{config_style, StaticStyles};
    use std::collections::HashMap;
    use wasm_bindgen::JsValue;
    use web_sys::Element;

    pub enum Styles {
        Static(StaticStyles),
        Dynamic(StyleToken),
    }

    impl Default for Styles {
        fn default() -> Self {
            Self::Static(Default::default())
        }
    }

    impl From<StaticStyles> for Styles {
        fn from(src: StaticStyles) -> Self {
            Self::Static(src)
        }
    }

    impl From<StyleToken> for Styles {
        fn from(src: StyleToken) -> Self {
            Self::Dynamic(src)
        }
    }

    impl Styles {
        pub fn val(&self) -> HashMap<String, String> {
            match self {
                Self::Static(val) => val.0.clone(),
                Self::Dynamic(val) => val.val().0,
            }
        }

        pub fn config_element(&self, ele: &Element) -> Result<(), JsValue> {
            match self {
                Self::Static(val) => {
                    config_style(&val, ele)?;
                }
                Self::Dynamic(token) => match token {
                    StyleToken::Owned { view, subs } => {
                        let subs = subs.clone();
                        let view = view.replace(Box::new(|| StaticStyles::default()));
                        let static_styles = view();
                        config_style(&static_styles, ele)?;
                        subs.borrow_mut()
                            .push((ele.clone(), StyleFunc::Owned(view)));
                    }
                    StyleToken::Shared { view, subs } => {
                        let static_styles = view.borrow()();
                        config_style(&static_styles, ele)?;
                        for item in subs.iter() {
                            item.borrow_mut()
                                .push((ele.clone(), StyleFunc::Shared(view.clone())))
                        }
                    }
                },
            }
            Ok(())
        }
    }

    // /// simple wrapper of tag style
    // #[derive(Debug, Clone, Default)]
    // pub struct Styles(pub HashMap<String, String>);

    /// helper macro to create css style
    ///
    /// ```no_run
    /// let s = style! {
    ///     background-color: "#fffff";
    ///     bar: "bxx";
    ///     font-size: 100;
    ///     escaped: r#""abc""#
    /// };
    /// ```
    /// generated
    /// ```css
    /// background-color: #fffff;
    /// bar: bxx;
    /// font-size: 100;
    /// escaped: "abc";
    /// ```
    #[macro_export]
    macro_rules! style {
        ($($($name:ident)-+: $value:expr);+ $(;)?) => {
            {
                let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                $(
                    let name = vec![$(stringify!($name)),*];
                    map.insert(name.join("-"), $value.to_string());
                )+
                $crate::tags::Styles::Static($crate::tags::StaticStyles(map))
            }
        };
        () => {
            $crate::tags::Styles::default()
        }
    }
}

pub use style::*;

impl From<Vec<(String, String)>> for StaticStyles {
    fn from(src: Vec<(String, String)>) -> Self {
        Self(src.into_iter().collect())
    }
}

#[doc(hidden)]
pub fn _warning(msg: &str) {
    tracing::warn!(msg)
}

/// simple wrapper of tag attributes
#[derive(Debug, Clone, Default)]
pub struct StaticAttrs(pub HashMap<String, String>);

pub(crate) fn config_attr(
    attrs: &StaticAttrs,
    ele: &web_sys::Element,
) -> Result<(), wasm_bindgen::JsValue> {
    for (name, value) in attrs.0.iter() {
        ele.set_attribute(name, value).map_err(|e| {
            tracing::error!("failed to set attribute {}=\"{}\", {:?}", name, value, e);
            e
        })?;
    }
    Ok(())
}
#[cfg(target_family = "wasm")]
mod attr {
    use crate::{AttrFunc, AttrToken};

    use super::{config_attr, StaticAttrs};
    use std::collections::HashMap;

    pub enum Attrs {
        Static(StaticAttrs),
        Dynamic(AttrToken),
    }

    impl Default for Attrs {
        fn default() -> Self {
            Self::Static(Default::default())
        }
    }

    impl From<StaticAttrs> for Attrs {
        fn from(src: StaticAttrs) -> Self {
            Self::Static(src)
        }
    }

    impl From<AttrToken> for Attrs {
        fn from(src: AttrToken) -> Self {
            Self::Dynamic(src)
        }
    }

    impl Attrs {
        pub fn val(&self) -> HashMap<String, String> {
            match self {
                Self::Static(val) => val.0.clone(),
                Self::Dynamic(val) => val.val().0,
            }
        }

        pub fn config_element(&self, ele: &web_sys::Element) -> Result<(), wasm_bindgen::JsValue> {
            match self {
                Self::Static(val) => {
                    config_attr(&val, ele)?;
                }
                Self::Dynamic(token) => match token {
                    AttrToken::Owned { view, subs } => {
                        let subs = subs.clone();
                        let view = view.replace(Box::new(|| StaticAttrs::default()));
                        let static_attrs = view();
                        config_attr(&static_attrs, ele)?;
                        subs.borrow_mut().push((ele.clone(), AttrFunc::Owned(view)));
                    }
                    AttrToken::Shared { view, subs } => {
                        let static_attrs = view.borrow()();
                        config_attr(&static_attrs, ele)?;
                        for item in subs.iter() {
                            item.borrow_mut()
                                .push((ele.clone(), AttrFunc::Shared(view.clone())))
                        }
                    }
                },
            }
            Ok(())
        }
    }

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
            $crate::tags::Attrs::Static($crate::tags::StaticAttrs(attrs))
        }};
        () => {{
            $crate::tags::Attrs::default()
        }}
    }
}

#[cfg(not(target_family = "wasm"))]
mod attr {
    use std::collections::HashMap;

    use super::{config_attr, StaticAttrs};

    pub type Attrs = StaticAttrs;

    impl Attrs {
        pub fn val(&self) -> HashMap<String, String> {
            self.0.clone()
        }

        pub fn config_element(&self, ele: &web_sys::Element) -> Result<(), wasm_bindgen::JsValue> {
            config_attr(&self, ele)
        }
    }

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
            $crate::tags::StaticAttrs(attrs)
        }};
        () => {{
            $crate::tags::StaticAttrs::default()
        }}
    }
}

pub use attr::*;
