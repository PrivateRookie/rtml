use rtml::{prop, style, tags::*, wasm::WasmRender, Tag};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let content: Box<dyn Tag> = Box::new(div((
        h1((
            prop! {id="big", class="a b c d e f"},
            style! {color: "red"},
            "hello",
        )),
        hr(()),
        p(1usize),
    )));
    if let Err(e) = content.as_ref().render(&body, &document) {
        debug(&format!("{:?}", e))
    }
    debug("render ok");

    Ok(())
}
