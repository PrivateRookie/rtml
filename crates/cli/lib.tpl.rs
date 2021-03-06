use rtml::tags::*;
use rtml::{t_attr, mount_body, t_style};
use rtml_project::debug_auto_reload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    debug_auto_reload();

    let page = div((
        t_style! {text-align: "center"},
        (
            h1("hello World!"),
            br(()),
            strong(a((
                t_attr! {href="http://"},
                "Power By https://github.com/PrivateRookie/rtml",
            ))),
        ),
    ));

    mount_body(page).unwrap();
}
