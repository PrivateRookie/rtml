use std::ops::Add;

use js_sys::Reflect;
use rtml::EventKind::{self, Click};
use rtml::{attr, mount_body, style, tags::*, IntoReactive};
use wasm_bindgen::prelude::*;

struct Item {
    id: usize,
    msg: String,
    done: bool,
}

impl Item {
    fn new<S: Into<String>>(id: usize, msg: S) -> Self {
        Self {
            id,
            msg: msg.into(),
            done: false,
        }
    }

    fn toggle(&mut self) {
        self.done = !self.done;
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    let items = vec![
        Item::new(1, "eat"),
        Item::new(2, "sleep"),
        Item::new(3, "coding"),
    ]
    .reactive();

    let msg = String::new().reactive();
    let new_item = form((
        h2("New Record"),
        input(attr! {type="text", id="xpr"}).on(
            EventKind::Change,
            msg.evt(|evt, msg| {
                if let Some(target) = evt.target() {
                    let value = Reflect::get(&target, &JsValue::from_str("value")).unwrap();
                    if let Some(val) = value.as_string() {
                        *msg.val_mut() = val;
                    }
                }
            }),
        ),
        button("+").on(
            Click,
            msg.add(items.clone()).evt(|event, (msg, items)| {
                event.prevent_default();
                if msg.val().is_empty() {
                } else {
                    let text = msg.val().clone();
                    items.val_mut().push(Item::new(100, text));
                }
            }),
        ),
    ));
    let cards = div((
        attr! {class="out-most"},
        items.view(|cards| {
            cards
                .val()
                .iter()
                .enumerate()
                .map(|(idx, data)| {
                    div((
                        attr! {class="container"},
                        (
                            p(format!("ID: {}", data.id)),
                            p(format!("Content: {}", data.msg)),
                            p(format!("status: {}", data.done)),
                            button((
                                style! {
                                    margin: "0 5px 0 0"
                                },
                                "toggle",
                            ))
                            .on(
                                Click,
                                cards.change(move |records| {
                                    if let Some(item) = records.val_mut().get_mut(idx) {
                                        item.toggle();
                                    }
                                }),
                            ),
                            button((
                                style! {
                                    margin: "0 5px 0 0"
                                },
                                "delete",
                            ))
                            .on(
                                Click,
                                cards.change(move |records| {
                                    records.val_mut().remove(idx);
                                }),
                            ),
                            hr(()),
                        ),
                    ))
                })
                .collect::<Vec<_>>()
                .into()
        }),
    ));
    mount_body(div((new_item, hr(()), cards))).unwrap();
}
