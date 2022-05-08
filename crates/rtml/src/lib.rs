use tags::{TagFormatter, TagProp, TagStyle};

mod basic_impl;
/// js relative
pub mod script;
/// css relative
pub mod style;
/// built in standard html tags
pub mod tags;

#[cfg(feature = "wasm")]
pub mod wasm;

pub type TagList = Vec<Box<dyn Tag>>;

/// helper struct for easier input
pub struct Children(pub TagList);

impl Children {
    pub fn push<T: Tag + 'static>(&mut self, item: T) {
        self.0.push(Box::new(item))
    }

    pub fn pop(&mut self) -> Option<Box<dyn Tag>> {
        self.0.pop()
    }
}

impl std::convert::AsMut<TagList> for Children {
    fn as_mut(&mut self) -> &mut TagList {
        &mut self.0
    }
}

impl std::convert::AsRef<TagList> for Children {
    fn as_ref(&self) -> &TagList {
        &self.0
    }
}

pub enum TagContent<'a> {
    Tags(&'a TagList),
    Text(String),
}

/// a trait represent html tags
pub trait Tag: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn props(&self) -> Option<&TagProp>;
    fn styles(&self) -> Option<&TagStyle>;
    fn content(&self) -> TagContent;
    fn format(&self, f: &mut TagFormatter, buf: &mut String) -> std::fmt::Result;
}

/// represent empty children
#[derive(Debug, Clone, Copy)]
pub struct Kong;
