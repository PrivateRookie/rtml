use rtml::{
    attr, mount_body, style,
    tags::{button, div, h1, i, input, p, Div},
    EventKind::Click,
    Marker,
};
use wasm_bindgen::prelude::*;
use web_sys::DateTimeValue;

mod bs;
use bs::*;

struct TodoRecord {
    text: String,
    done: bool,
    create_dt: DateTimeValue,
    update_dt: DateTimeValue,
}

impl TodoRecord {
    pub fn new(text: String) -> Self {
        let now = DateTimeValue::new();
        Self {
            text,
            done: false,
            create_dt: now.clone(),
            update_dt: now,
        }
    }

    /// toggle done field, return updated done value
    pub fn toggle_status(&mut self) -> bool {
        self.done = !self.done;
        self.update_dt = DateTimeValue::new();
        self.done
    }

    /// change
    pub fn change_text(&mut self, text: String) -> String {
        let old = std::mem::replace(&mut self.text, text);
        self.update_dt = DateTimeValue::new();
        old
    }
}

fn record(item: &TodoRecord) -> Div {
    let (icon, btn_label) = if item.done {
        ("bi bi-check-circle", "Undone")
    } else {
        ("bi bi-circle", "Done")
    };
    row_div((
        col_div(10, p(&item.text)),
        col_div(1, i(attr! {class=icon})),
        col_div(1, button(btn_label)),
    ))
}

fn items() -> Div {
    let items = vec![
        TodoRecord::new("eat".into()),
        TodoRecord::new("writing".into()),
    ];
    let children = items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            record(item).inject(idx).on(Click, |line| {
                Box::new(move || {
                    let id = line.data.borrow();
                    tracing::info!("{} is clicked", id);
                })
            })
        })
        .collect::<Vec<_>>();
    div((h1("things"), div(children)))
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    let item = div((h1("TODO"), items(), div((input(()), button("Add")))));
    if let Err(e) = mount_body(item) {
        tracing::error!("failed to init page dom, {:?}", e);
    }
}
