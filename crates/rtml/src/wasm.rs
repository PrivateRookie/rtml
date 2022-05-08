use web_sys::{Document, Element};

use wasm_bindgen::prelude::*;

use crate::{Tag, TagContent};

pub trait WasmRender {
    fn render(&self, parent: &Element, doc: &Document) -> Result<(), JsValue>;
}

macro_rules! impl_render {
    ($self:ident, $parent:ident, $doc:ident) => {{
        match $self.content() {
            TagContent::Text(value) => {
                $parent.set_text_content(Some(&value));
            }
            TagContent::Tags(children) => {
                let ele = $doc.create_element($self.name())?;
                if let Some(props) = $self.props() {
                    for (name, value) in props.0.iter() {
                        ele.set_attribute(name, value)?;
                    }
                }
                if let Some(styles) = $self.styles() {
                    let styles = styles
                        .0
                        .iter()
                        .map(|(k, v)| format!("{k}:{v}"))
                        .collect::<Vec<_>>()
                        .join(";");
                    ele.set_attribute("style", &styles)?;
                }
                for child in children {
                    child.as_ref().render(&ele, $doc)?;
                }
                $parent.append_child(&ele)?;
            }
        }
        Ok(())
    }};
}

impl WasmRender for &dyn Tag {
    fn render(&self, parent: &Element, doc: &Document) -> Result<(), JsValue> {
        impl_render!(self, parent, doc)
    }
}

impl<T: Tag> WasmRender for &T {
    fn render(&self, parent: &Element, doc: &Document) -> Result<(), JsValue> {
        impl_render!(self, parent, doc)
    }
}
