use rtml::EventKind::Click;
use rtml::{attr, mount_body, tags::*};
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
                            button("toggle").on(
                                Click,
                                cards.evt(move |_, records| {
                                    if let Some(item) = records.val_mut().get_mut(idx) { item.toggle(); }
                                }),
                            ),
                            button("delete").on(
                                Click,
                                cards.evt(move |_, records| {
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
    mount_body(cards).unwrap();
}
