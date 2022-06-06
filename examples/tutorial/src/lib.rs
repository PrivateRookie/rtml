use rtml::{mount_body, ref_subs, tags::*, EventKind::Click, IntoReactive};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn reactive_view() {
    tracing_wasm::set_as_global_default();

    // wrap data with a reactive struct
    let count = 0usize.reactive();

    // create a dynamic view from data
    // this view will be call when data change
    let btn_label = count.view(|data| {
        // use val to get wrapped data
        let count = data.val();
        let s = if *count > 1 { "s" } else { "" };
        // return value can be string or single html tag
        format!("Clicked {count} time{s}")
    });

    // another view
    let doubled = count.view(|data| {
        let count = data.val();
        let double = *count * 2;
        format!("{count} doubled is {double}")
    });

    // use change event to create data mutable closure
    // if this closure is called, all relative view will be called
    // to update content
    let incr = count.change(|data| {
        *data.val_mut() += 1;
        true
    });

    // view can be pass like normal content
    let btn = button(btn_label)
        // use on method to bind event listener
        .on(Click, incr);

    // render and mount to body
    mount_body(div((
        // combine paragraph and button
        p(doubled),
        btn,
    )))
    .unwrap();
}

#[wasm_bindgen]
pub fn reactive_if_else() {
    tracing_wasm::set_as_global_default();

    struct User {
        logged_in: bool,
    }

    let user = User { logged_in: false }.reactive();

    let toggle = user.change(|data| {
        let mut user = data.val_mut();
        user.logged_in = !user.logged_in;
        true
    });

    let btn = button(ref_subs!(user => if user.val().logged_in {
        "Log out"
    } else {
        "Log in"
    }))
    .on(Click, toggle);

    mount_body(btn).unwrap();
}
