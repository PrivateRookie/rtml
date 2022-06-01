use rtml::tags::*;
use rtml::{attr, mount_body, style};
use rtml_project::debug_auto_reload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    debug_auto_reload();

    let page = div((
        style! {text-align: "center"},
        (
            h1("hello World!"),
            br(()),
            strong(a((
                attr! {href="http://"},
                "Power By https://github.com/PrivateRookie/rtml",
            ))),
        ),
    ));

    mount_body(page).unwrap();
}
