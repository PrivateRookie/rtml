use tags::TagFormatter;

mod basic_impl;
/// js relative
pub mod script;
/// css relative
pub mod style;
/// built in standard html tags
pub mod tags;

pub type InnerChildren = Vec<Box<dyn Tag>>;

/// helper struct for easier input
pub struct Children(pub InnerChildren);

/// a trait represent html tags
pub trait Tag {
    fn name(&self) -> &'static str;
    fn format(&self, f: &mut TagFormatter, buf: &mut String) -> std::fmt::Result;
}

/// represent empty children
#[derive(Clone, Copy)]
pub struct Kong;
