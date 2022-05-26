use rtml::{
    mount_body,
    tags::{button, div, p},
    EventKind::Click,
    IntoReactive,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let count = 0usize.reactive();
    let display = count.view(|data| {
        let count = data.val();
        let content = if *count >= 1 {
            format!("count {count} times")
        } else {
            format!("count {count} time")
        };
        content.into()
    });

    let incr = count.change(|data| {
        *data.val_mut() += 1;
        true
    });

    let dec = count.change(|data| {
        if *data.val() > 0 {
            *data.val_mut() -= 1;
            true
        } else {
            false
        }
    });
    let counter = div((
        p(display),
        button("+1").on(Click, incr),
        button("-1").on(Click, dec),
    ));

    mount_body(counter).unwrap();
}
