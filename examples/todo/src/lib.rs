use std::ops::Deref;

use rtml::EventKind::{Blur, Click, DblClick, Keypress};
use rtml::{t_attr, mount_body, s_attr, t_style, tags::*, IntoReactive, Reactive};
use store::Store;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, KeyboardEvent};

mod store;

#[derive(Debug, Clone, PartialEq, Eq)]
enum FilterStatus {
    All,
    Active,
    Completed,
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    let records = store::Store::load().reactive();
    let filter = FilterStatus::Active.reactive();

    let toggle_all = records.change(|data| {
        data.val_mut()
            .items
            .iter_mut()
            .for_each(|item| item.completed = !item.completed);
        data.val().save().unwrap();
        true
    });

    let clear_complete = records.change(|data| {
        data.val_mut().clear_completed();
        true
    });

    let wrapper = div((
        t_attr! {class="todomvc-wrapper"},
        (
            section((
                t_attr! {class="todoapp"},
                (
                    header((
                        t_attr! {class="header"},
                        (
                            h1("RTML - Todos"),
                            input_view(records.clone()),
                        ),
                    )),
                    section((
                        records.attr(|data| {
                            if data.val().items.is_empty() {
                                s_attr! { class= "main hidden"}
                            } else {
                                s_attr! { class="main"}
                            }
                        }),
                        (
                            input(records.attr(|data| {
                                let checked = data.val().items.iter().all(|i| i.completed);
                                s_attr! { type="checkbox", class="toggle-all", id="toggle-all", checked=checked }
                            }))
                            .on(Click, toggle_all),
                            label(t_attr! {for="toggle-all"}),
                            todo_list(records.clone(), filter.clone()),
                        ),
                    )),
                    footer((
                        records.attr(|data| {
                            let cls = if data.val().items.is_empty() {
                                "footer hidden"
                            } else {
                                "footer"
                            };
                            s_attr! {class=cls}
                        }),
                        (span((
                            t_attr! {class="todo-count"},
                            (
                                strong(records.view(|data| data.val().items.len())),
                                span(" item(s) left"),
                            )
                        )),
                        filter_view(filter),
                        button((t_attr!{class="clear-completed"}, records.view(|data| {
                            format!("Clear Completed {}", data.val().items.iter().filter(|i| i.completed).count())
                        }))).on(Click, clear_complete)
                    ),
                    )),
                ),
            )),
            footer((
                t_attr! {class="info"},
                (
                    p("Double-click to edit a todo"),
                    p((
                        span("Written by "),
                        a((
                            t_attr! {href="https://github.com/PrivateRookie", target="_blank"},
                            "PrivateRookie",
                        )),
                    )),
                    p((
                        span("Part of "),
                        a((
                            t_attr! {href="http://todomvc.com/", target="_blank"},
                            "TodoMVC",
                        )),
                    )),
                ),
            )),
        ),
    ));

    mount_body(wrapper).unwrap();
}

fn input_view(records: Reactive<Store>) -> Input {
    let update_and_reset = records.evt(|event, data| {
        let event: KeyboardEvent = JsValue::from(event).into();
        let mut update = false;
        if event.key() == "Enter" {
            if let Some(target) = event.target() {
                let input: HtmlInputElement = JsValue::from(target).into();
                let value = input.value();
                data.val_mut().add(value);
                input.set_value("");
                update = true;
            }
        }
        update
    });
    input(t_attr! {id="edit-input",class="new-todo", placeholder="What needs to be done?"})
        .on(Keypress, update_and_reset)
}

fn filter_view(filter: Reactive<FilterStatus>) -> Ul {
    use FilterStatus::*;
    let options = vec![All, Completed, Active];
    ul((
        t_attr! {class="filters"},
        options
            .into_iter()
            .map(|opt| {
                let s = format!("{:?}", opt);
                let opt_c = opt.clone();
                li(a((
                    filter.attr(move |data| {
                        let cls = if *data.val() == opt_c {
                            "selected"
                        } else {
                            "not-selected"
                        };
                        s_attr! { class=cls }
                    }),
                    s,
                ))
                .on(
                    Click,
                    filter.change(move |data| {
                        *data.val_mut() = opt.clone();
                        true
                    }),
                ))
            })
            .collect::<Vec<_>>(),
    ))
}

fn todo_list(records: Reactive<Store>, filter: Reactive<FilterStatus>) -> Ul {
    let combine = records + filter;
    ul(
        combine.view(|(records, filter)| {
            let opt = filter.val();
            let opt = opt.deref();
            records
                .val()
                .items
                .iter()
                .filter(|item| match opt {
                    FilterStatus::All => true,
                    FilterStatus::Active => !item.completed,
                    FilterStatus::Completed => item.completed,
                })
                .enumerate()
                .map(|(idx, item)| {
                    let mut input_attr = s_attr! { type="checkbox", class="toggle" };
                    if item.completed {
                        input_attr.0.insert("checked".into(), "".into());
                    }
                    li((
                        div((
                            t_attr! {class="view"},
                            (
                                input(Attrs::Static(input_attr)).on(
                                    Click,
                                    records.change(move |store| {
                                        store.val_mut().toggle(idx);
                                        true
                                    }),
                                ),
                                label(&item.description).on(
                                    DblClick,
                                    records.change(move |store| {
                                        if let Some(item) = store.val_mut().items.get_mut(idx) {
                                            item.editing = !item.editing;
                                        }
                                        store.val().save().unwrap();
                                        true
                                    }),
                                ),
                                button((
                                    t_attr! {class="destroy"},
                                    t_style! {margin: "10px"},
                                    "🚮",
                                ))
                                .on(
                                    Click,
                                    records.change(move |store| {
                                        store.val_mut().remove(idx);
                                        true
                                    }),
                                ),
                            ),
                        )),
                        item_edit_input(item, idx, records.clone()),
                    ))
                })
                .collect::<Vec<_>>()
        }),
    )
}

fn item_edit_input(item: &store::Item, idx: usize, data: Reactive<Store>) -> Input {
    fn edit(event: web_sys::Event, store: Reactive<Store>, idx: usize) {
        let event: KeyboardEvent = JsValue::from(event).into();
        if let Some(target) = event.target() {
            let input: HtmlInputElement = JsValue::from(target).into();
            let value = input.value();
            store.val_mut().edit(idx, value);
            input.set_value("");
        }
    }

    if item.editing {
        let on_blur = data.evt(move |event, store| {
            edit(event, store, idx);
            true
        });
        input(t_attr! {class="edit", type="text", value=item.description}).on(Blur, on_blur)
    } else {
        input(t_attr! {type="hidden"})
    }
}
