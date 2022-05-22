#[macro_export]
/// def a thin wrapper of html element
macro_rules! def {
    ($func_name:ident, $struct:ident, $arg:ident, doc: $($lang:literal = $doc:literal);+) => {
        $(#[cfg_attr(feature=$lang, doc=$doc)])+
        pub fn $func_name<T: Into<$arg>>(tag: T) -> $struct {
            let args : $arg = tag.into();
            let $arg { content, attrs, styles } = args;
            $struct($crate::tags::Unit {
                name: stringify!($func_name),
                content,
                attrs,
                styles,
                listeners: Default::default()
            })
        }

        $(#[cfg_attr(feature=$lang, doc=$doc)])+
        pub struct $struct(pub $crate::tags::Unit);

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl $struct {
            // /// update component style with passed styles
            // pub fn s(mut self, styles: $crate::tags::Styles) -> Self {
            //     styles.0.into_iter().for_each(|(key, value)| {
            //         self.0.styles.0.insert(key, value);
            //     });
            //     self
            // }

            // /// replace origin component style
            // pub fn s_replace(mut self, styles: $crate::tags::Styles) -> Self {
            //     self.0.styles = styles;
            //     self
            // }

            // /// update component attributes with passed attrs
            // pub fn a(mut self, attrs: $crate::tags::Attrs) -> Self {
            //     attrs.0.into_iter().for_each(| (key, value)| {
            //         self.0.attrs.0.insert(key, value);
            //     });
            //     self
            // }

            // /// replace component attributes with passed attrs
            // pub fn a_replace(mut self, attrs: $crate::tags::Attrs) -> Self {
            //     self.0.attrs = attrs;
            //     self
            // }

            /// set element children
            pub fn children<C: Into<$crate::tags::EleContent>>(mut self, children: C) -> Self {
                self.0.content = children.into();
                self
            }

            /// add event listeners
            pub fn on<K: Into<&'static str>>(self, kind: K, listener: Box<dyn Fn() -> Box<dyn Fn(web_sys::Event)>>) -> Self {
                $struct(self.0.on(kind, listener))
            }
        }

        impl $crate::Template for $struct {
            fn resources(& self) -> $crate::TplResources {
                self.0.resources()
            }

        }

        pub struct $arg {
            pub content: $crate::tags::EleContent,
            pub attrs: $crate::tags::Attrs,
            pub styles: $crate::tags::Styles,
        }

        impl<C: Into<$crate::tags::EleContent>> From<C> for $arg {
            fn from(src:  C) -> Self {
                Self {
                    content: src.into(),
                    attrs: Default::default(),
                    styles: Default::default(),
                }
            }
        }

        impl<C: Into<$crate::tags::EleContent>> From<($crate::tags::Attrs, C)> for $arg {
            fn from(src: ( $crate::tags::Attrs, C)) -> Self {
                Self {
                    content: src.1.into(),
                    attrs: src.0,
                    styles: Default::default(),
                }
            }
        }


        impl<C: Into<$crate::tags::EleContent>> From<($crate::tags::Styles, C)> for $arg {
            fn from(src: ( $crate::tags::Styles, C)) -> Self {
                Self {
                    content: src.1.into(),
                    attrs: Default::default(),
                    styles: src.0,
                }
            }
        }

        impl<C: Into<$crate::tags::EleContent>> From<($crate::tags::Attrs, $crate::tags::Styles, C)> for $arg {
            fn from(src: ( $crate::tags::Attrs, $crate::tags::Styles, C)) -> Self {
                Self {
                    content: src.2.into(),
                    attrs: src.0,
                    styles: src.1,
                }
            }
        }
        impl<C: Into<$crate::tags::EleContent>> From<($crate::tags::Styles, $crate::tags::Attrs, C)> for $arg {
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

mod a;
pub use a::*;

mod abbr;
pub use abbr::*;

mod acronym;
pub use acronym::*;

mod address;
pub use address::*;

mod applet;
pub use applet::*;

mod area;
pub use area::*;

mod article;
pub use article::*;

mod aside;
pub use aside::*;

mod audio;
pub use audio::*;

mod b;
pub use b::*;

mod base;
pub use base::*;

mod basefont;
pub use basefont::*;

mod bdi;
pub use bdi::*;

mod bdo;
pub use bdo::*;

mod bgsound;
pub use bgsound::*;

mod big;
pub use big::*;

mod blockquote;
pub use blockquote::*;

mod body;
pub use body::*;

mod br;
pub use br::*;

mod button;
pub use button::*;

mod canvas;
pub use canvas::*;

mod caption;
pub use caption::*;

mod cite;
pub use cite::*;

mod code;
pub use code::*;

mod col;
pub use col::*;

mod colgroup;
pub use colgroup::*;

mod content;
pub use content::*;

mod data;
pub use data::*;

mod datalist;
pub use datalist::*;

mod dd;
pub use dd::*;

mod del;
pub use del::*;

mod details;
pub use details::*;

mod dfn;
pub use dfn::*;

mod dialog;
pub use dialog::*;

mod dir;
pub use dir::*;

mod div;
pub use div::*;

mod dl;
pub use dl::*;

mod dt;
pub use dt::*;

mod em;
pub use em::*;

mod embed;
pub use embed::*;

mod fieldset;
pub use fieldset::*;

mod figcaption;
pub use figcaption::*;

mod figure;
pub use figure::*;

mod font;
pub use font::*;

mod footer;
pub use footer::*;

mod form;
pub use form::*;

mod frame;
pub use frame::*;

mod frameset;
pub use frameset::*;

mod head;
pub use head::*;

mod header;
pub use header::*;

mod h1;
pub use h1::*;

mod h2;
pub use h2::*;

mod h3;
pub use h3::*;

mod h4;
pub use h4::*;

mod h5;
pub use h5::*;

mod h6;
pub use h6::*;

mod hgroup;
pub use hgroup::*;

mod hr;
pub use hr::*;

mod html;
pub use html::*;

mod i;
pub use i::*;

mod iframe;
pub use iframe::*;

mod img;
pub use img::*;

mod input;
pub use input::*;

mod ins;
pub use ins::*;

mod kbd;
pub use kbd::*;

mod keygen;
pub use keygen::*;

mod label;
pub use label::*;

mod legend;
pub use legend::*;

mod li;
pub use li::*;

mod link;
pub use link::*;

mod main;
pub use main::*;

mod map;
pub use map::*;

mod mark;
pub use mark::*;

mod marquee;
pub use marquee::*;

mod menu;
pub use menu::*;

mod menuitem;
pub use menuitem::*;

mod meta;
pub use meta::*;

mod meter;
pub use meter::*;

mod nav;
pub use nav::*;

mod noframes;
pub use noframes::*;

mod noscript;
pub use noscript::*;

mod object;
pub use object::*;

mod ol;
pub use ol::*;

mod optgroup;
pub use optgroup::*;

mod option;
pub use option::*;

mod output;
pub use output::*;

mod p;
pub use p::*;

mod param;
pub use param::*;

mod picture;
pub use picture::*;

mod plaintext;
pub use plaintext::*;

mod portal;
pub use portal::*;

mod pre;
pub use pre::*;

mod progress;
pub use progress::*;

mod q;
pub use q::*;

mod rb;
pub use rb::*;

mod rp;
pub use rp::*;

mod rt;
pub use rt::*;

mod rtc;
pub use rtc::*;

mod ruby;
pub use ruby::*;

mod s;
pub use s::*;

mod samp;
pub use samp::*;

mod script;
pub use script::*;

mod section;
pub use section::*;

mod select;
pub use select::*;

mod shadow;
pub use shadow::*;

mod slot;
pub use slot::*;

mod small;
pub use small::*;

mod source;
pub use source::*;

mod spacer;
pub use spacer::*;

mod span;
pub use span::*;

mod strike;
pub use strike::*;

mod strong;
pub use strong::*;

mod style;
pub use style::*;

mod sub;
pub use sub::*;

mod summary;
pub use summary::*;

mod sup;
pub use sup::*;

mod table;
pub use table::*;

mod tbody;
pub use tbody::*;

mod td;
pub use td::*;

mod template;
pub use template::*;

mod textarea;
pub use textarea::*;

mod tfoot;
pub use tfoot::*;

mod th;
pub use th::*;

mod thead;
pub use thead::*;

mod time;
pub use time::*;

mod title;
pub use title::*;

mod tr;
pub use tr::*;

mod track;
pub use track::*;

mod tt;
pub use tt::*;

mod u;
pub use u::*;

mod ul;
pub use ul::*;

mod var;
pub use var::*;

mod video;
pub use video::*;

mod wbr;
pub use wbr::*;

mod xmp;
pub use xmp::*;
