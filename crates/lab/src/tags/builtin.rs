#[macro_export]
/// def a thin wrapper of html element
macro_rules! def {
    ($func_name:ident, $struct:ident, $arg:ident, doc: $($lang:literal = $doc:literal);+) => {
        $(#[cfg_attr(feature=$lang, doc=$doc)])+
        pub fn $func_name<T: Into<$arg>>(tag: T) -> $struct<($crate::Marker, )> {
            let args : $arg = tag.into();
            let $arg { content, attrs, styles } = args;
            $struct($crate::tags::Unit {
                name: stringify!($func_name),
                content,
                attrs,
                styles,
                markers: ($crate::Marker::default(), ),
                listeners: Default::default()
            })
        }

        $(#[cfg_attr(feature=$lang, doc=$doc)])+
        pub struct $struct<D: $crate::Markers>(pub $crate::tags::Unit<D>);

        impl<M: $crate::Markers + Clone> $crate::Template for $struct<M> {
            fn resources(&self) -> (
                &'static str,
                &$crate::tags::Content,
                std::collections::HashMap<&str, Box<dyn FnOnce() -> Box<dyn FnMut()> + '_>>,
            ) {
                self.0.resources()
            }

            fn set_element(&self, element: web_sys::Element) {
                self.0.set_element(element)
            }
        }

        pub struct $arg {
            pub content: $crate::tags::Content,
            pub attrs: $crate::tags::Attrs,
            pub styles: $crate::tags::Styles,
        }

        impl<C: Into<$crate::tags::Content>> From<C> for $arg {
            fn from(src:  C) -> Self {
                Self {
                    content: src.into(),
                    attrs: Default::default(),
                    styles: Default::default(),
                }
            }
        }

        impl<C: Into<$crate::tags::Content>> From<($crate::tags::Attrs, C)> for $arg {
            fn from(src: ( $crate::tags::Attrs, C)) -> Self {
                Self {
                    content: src.1.into(),
                    attrs: src.0,
                    styles: Default::default(),
                }
            }
        }


        impl<C: Into<$crate::tags::Content>> From<($crate::tags::Styles, C)> for $arg {
            fn from(src: ( $crate::tags::Styles, C)) -> Self {
                Self {
                    content: src.1.into(),
                    attrs: Default::default(),
                    styles: src.0,
                }
            }
        }

        impl<C: Into<$crate::tags::Content>> From<($crate::tags::Attrs, $crate::tags::Styles, C)> for $arg {
            fn from(src: ( $crate::tags::Attrs, $crate::tags::Styles, C)) -> Self {
                Self {
                    content: src.2.into(),
                    attrs: src.0,
                    styles: src.1,
                }
            }
        }
        impl<C: Into<$crate::tags::Content>> From<($crate::tags::Styles, $crate::tags::Attrs, C)> for $arg {
            fn from(src: ( $crate::tags::Styles, $crate::tags::Attrs, C)) -> Self {
                Self {
                    content: src.2.into(),
                    styles: src.0,
                    attrs: src.1,
                }
            }
        }


        impl From<$crate::tags::Attrs> for $arg {
            fn from(src: $crate::tags::Attrs) -> Self {
                Self {
                    content: Default::default(),
                    attrs: src,
                    styles: Default::default(),
                }
            }
        }

        impl From<$crate::tags::Styles> for $arg {
            fn from(src:  $crate::tags::Styles) -> Self {
                Self {
                    content: Default::default(),
                    attrs: Default::default(),
                    styles: src,
                }
            }
        }

        impl From<($crate::tags::Attrs, $crate::tags::Styles)> for $arg {
            fn from(src: ( $crate::tags::Attrs, $crate::tags::Styles)) -> Self {
                Self {
                    content: Default::default(),
                    attrs: src.0,
                    styles: src.1,
                }
            }
        }
        impl From<($crate::tags::Styles, $crate::tags::Attrs)> for $arg {
            fn from(src: ( $crate::tags::Styles, $crate::tags::Attrs)) -> Self {
                Self {
                    content: Default::default(),
                    styles: src.0,
                    attrs: src.1,
                }
            }
        }
    };
}
