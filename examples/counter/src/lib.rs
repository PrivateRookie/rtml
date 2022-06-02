use rtml::{
    mount_body, IntoReactive, {button, div, h2},
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
        content
    });

    let counter = div! { => (
        h2!{ => display },
        button!{
            @click = count =>|_| {
                *count.val_mut() += 1;
                true
            }
            => "+1"
        },
        button!{
            @click = count =>   |_| {
                if *count.val() > 0 {
                    *count.val_mut() -= 1;
                    true
                } else {
                    false
                }
            }
            => "-1"
        }
        )
    };

    mount_body(counter).unwrap();
}
