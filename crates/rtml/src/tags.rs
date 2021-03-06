use crate::{reactive::Dom, tag_fmt::TagFormatter, Listeners, Template, TemplateList};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

mod builtin;
pub use builtin::*;
use web_sys::{Document, Element, Event};

pub struct Unit {
    pub name: &'static str,
    pub content: EleContent,
    pub modifier: EleModifierRegisterData,
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
        &EleModifierRegisterData,
        &Attrs,
        &Styles,
        &EleContent,
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
            &self.modifier,
            &self.attrs,
            &self.styles,
            &self.content,
            other_factories,
        )
    }
}

impl Unit {
    pub fn bind(mut self, m: (Vec<Rc<RefCell<Dom>>>, crate::reactive::ModifierFunc)) -> Self {
        self.modifier = Some(m);
        self
    }

    pub fn on<K: Into<&'static str>>(
        mut self,
        kind: K,
        listener: Box<dyn Fn() -> Box<dyn Fn(Event)>>,
    ) -> Self {
        self.listeners.insert(kind.into(), listener);
        self
    }
}

/// html element content
pub enum StaticContent {
    /// empty tag
    Null,
    /// this element has children
    List(TemplateList),
    // TODO add html escape checking
    /// normal text content
    Text(String),
}

pub enum EleContent {
    Static(StaticContent),
    Dynamic {
        subs: Vec<Rc<RefCell<Dom>>>,
        func: Rc<RefCell<dyn Fn() -> StaticContent>>,
    },
}

impl Default for EleContent {
    fn default() -> Self {
        Self::Static(Default::default())
    }
}

impl Default for StaticContent {
    fn default() -> Self {
        Self::Null
    }
}

fn render_static(
    content: &StaticContent,
    path: Vec<usize>,
    ele: &web_sys::Element,
    doc: &Document,
) -> Result<(), wasm_bindgen::JsValue> {
    match content {
        StaticContent::Null => {}
        StaticContent::Text(text) => {
            ele.set_inner_html(text);
        }
        StaticContent::List(children) => {
            for (idx, child) in children.iter().enumerate() {
                let mut child_path = path.clone();
                child_path.push(idx);
                let child = child.render(child_path, ele, doc)?;
                ele.append_child(&child)?;
            }
        }
    }
    Ok(())
}

pub type ContentRegisterData = Option<(
    Vec<Rc<RefCell<Dom>>>,
    Rc<RefCell<dyn Fn() -> StaticContent>>,
)>;

impl EleContent {
    pub fn render(
        &self,
        path: Vec<usize>,
        ele: &web_sys::Element,
        doc: &Document,
    ) -> Result<ContentRegisterData, wasm_bindgen::JsValue> {
        let ret = match self {
            EleContent::Static(content) => {
                render_static(content, path, ele, doc)?;
                None
            }
            EleContent::Dynamic { subs, func } => {
                let content = func.borrow()();
                render_static(&content, path, ele, doc)?;
                Some((subs.clone(), func.clone()))
            }
        };
        Ok(ret)
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

pub enum Styles {
    Static(StaticStyles),
    Dynamic {
        subs: Vec<Rc<RefCell<Dom>>>,
        func: Rc<RefCell<dyn Fn() -> StaticStyles>>,
    },
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

pub type StyleRegisterData = Option<(Vec<Rc<RefCell<Dom>>>, Rc<RefCell<dyn Fn() -> StaticStyles>>)>;

impl Styles {
    pub fn val(&self) -> HashMap<String, String> {
        match self {
            Self::Static(val) => val.0.clone(),
            Self::Dynamic { func, .. } => func.borrow()().0,
        }
    }

    pub fn config_element(
        &self,
        ele: &web_sys::Element,
    ) -> Result<StyleRegisterData, wasm_bindgen::JsValue> {
        let ret = match self {
            Self::Static(val) => {
                config_style(val, ele)?;
                None
            }
            Self::Dynamic { subs, func } => {
                let static_style = func.borrow()();
                config_style(&static_style, ele)?;
                Some((subs.clone(), func.clone()))
            }
        };
        Ok(ret)
    }
}

/// helper macro to create css style
///
/// ```no_run
/// let s = t_style! {
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
///
/// to subscribe reactive data, explicit add reactive data ident, and add "*>" symbol
///
/// ```no_run
/// let age = 0u8.reactive();
/// let s = t_style! { age *>
///     background-color: if age.val() > 60 { "red" } else { "green" };
///     bar: "bxx";
///     font-size: 100;
///     escaped: r#""abc""#
/// }
/// ```
#[macro_export]
macro_rules! t_style {
    ($($d:ident),+ *> $($($name:ident)-+: $value:expr);+ $(;)?) => {
        {
            $(let $d = $d.clone();)+
            $crate::tags::Styles::Dynamic {
                subs: vec![$($d.subs.clone()),+],
                func: ::std::rc::Rc::new(::std::cell::RefCell::new(move|| $crate::s_style! {
                    $($($name)-+ : $value);+
                }))
            }
        }
    };
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

/// helper macro to create css style
///
/// ```no_run
/// let s = t_style! {
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
macro_rules! s_style {
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
        Default::default()
    }
}
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
    let old_attrs = ele.attributes();
    let mut old_keys = (0..old_attrs.length())
        .filter_map(|idx| {
            old_attrs.get_with_index(idx).and_then(|attr| {
                let name = attr.name();
                if name == "style" {
                    None
                } else {
                    Some(attr.name())
                }
            })
        })
        .collect::<HashSet<_>>();
    for (name, value) in attrs.0.iter() {
        let old = old_attrs.get_with_name(name).map(|a| a.value());
        if old.map(|v| v != *value).unwrap_or(true) {
            ele.set_attribute(name, value).map_err(|e| {
                tracing::error!("failed to set attribute {}=\"{}\", {:?}", name, value, e);
                e
            })?;
        }
        old_keys.remove(name);
    }
    for to_delete in old_keys {
        ele.remove_attribute(&to_delete).map(|e| {
            tracing::error!("failed to remove attribute {}", to_delete);
            e
        })?;
    }
    Ok(())
}

pub enum Attrs {
    Static(StaticAttrs),
    Dynamic {
        subs: Vec<Rc<RefCell<Dom>>>,
        func: Rc<RefCell<dyn Fn() -> StaticAttrs>>,
    },
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

pub type AttrRegisterData = Option<(Vec<Rc<RefCell<Dom>>>, Rc<RefCell<dyn Fn() -> StaticAttrs>>)>;

impl Attrs {
    pub fn val(&self) -> HashMap<String, String> {
        match self {
            Self::Static(val) => val.0.clone(),
            Self::Dynamic { func, .. } => func.borrow()().0,
        }
    }

    pub fn config_element(
        &self,
        ele: &web_sys::Element,
    ) -> Result<AttrRegisterData, wasm_bindgen::JsValue> {
        let ret = match self {
            Self::Static(val) => {
                config_attr(val, ele)?;
                None
            }
            Self::Dynamic { subs, func } => {
                let static_attrs = func.borrow()();
                config_attr(&static_attrs, ele)?;
                Some((subs.clone(), func.clone()))
            }
        };
        Ok(ret)
    }
}

/// helper macro to set html element attributes, like id, class, hidden etc
///
/// ```no_run
/// div((
///     t_attr! { id="app", data-some-demo },
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
/// **NOTE**: style should be set by `t_style!` macro
///
/// to subscribe reactive data, explicit add data identifier and add "#>"
///
/// ```no_run
/// t_attr! { age #> value=age.val() }
/// ```
#[macro_export]
macro_rules! t_attr {
    ($($d:ident),+ #> $($($name:tt)-+ $(= $value:expr)?),+ $(,)?) => {
        {
            $(let $d = $d.clone();)+
            $crate::tags::Attrs::Dynamic {
                subs: vec![$($d.subs.clone()),+],
                func: ::std::rc::Rc::new(::std::cell::RefCell::new(move|| $crate::s_attr! {
                    $($($name)-+ $(= $value)?),+
                }))
            }
        }
    };
    ($($($name:tt)-+ $(= $value:expr)?),+ $(,)?) => {
        { let mut attrs: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        $(
            let name = vec![$(stringify!($name)),*];
            let key = name.join("-");
            let mut valid = true;
            if key.starts_with("on") {
                $crate::tags::_warning("event handler can not be set with t_attr! macro");
                valid = false;
            } else if key == "style" {
                $crate::tags::_warning("style should be set with t_style! macro");
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

/// helper macro to set html element attributes, like id, class, hidden etc
///
/// ```no_run
/// div((
///     t_attr! { id="app", data-some-demo },
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
/// **NOTE**: style should be set by `t_style!` macro
#[macro_export]
macro_rules! s_attr {
    ($($($name:tt)-+ $(= $value:expr)?),+ $(,)?) => {
        { let mut attrs: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        $(
            let name = vec![$(stringify!($name)),*];
            let key = name.join("-");
            let mut valid = true;
            if key.starts_with("on") {
                $crate::tags::_warning("event handler can not be set with t_attr! macro");
                valid = false;
            } else if key == "style" {
                $crate::tags::_warning("style should be set with t_style! macro");
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
        Default::default()
    }}
}

pub type EleModifierRegisterData = Option<(Vec<Rc<RefCell<Dom>>>, Rc<RefCell<dyn Fn(Element)>>)>;
