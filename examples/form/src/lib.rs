use rtml::{
    mount_body,
    tags::{div, h2, hr},
};

// mod test;
mod func_style;
mod macro_style;
use rtml_project::debug_auto_reload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();
    debug_auto_reload();
    let forms = div((
        h2("func style"),
        func_style::func_form(),
        hr(()),
        h2("macro style"),
        macro_style::macro_form(),
    ));
    mount_body(forms).unwrap();
}
