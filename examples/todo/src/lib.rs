use rtml::EventKind::{Blur, Click, Keypress, Mouseover};
use rtml::{attr, mount_body, tags::*, IntoReactive, Reactive};
use store::Store;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, KeyboardEvent};

mod store;

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    let records = store::Store::load().reactive();

    let toggle_all = records.change(|data| {
        data.val_mut()
            .items
            .iter_mut()
            .for_each(|item| item.completed = !item.completed);
        data.val().save().unwrap();
    });

    let wrapper = div((
        attr! {class="todomvc-wrapper"},
        (
            section((
                attr! {class="todoapp"},
                (
                    header((
                        attr! {class="header"},
                        (
                            h1("RTML - Todos"),
                            input_view(records.clone()),
                        ),
                    )),
                    section((
                        attr! {class="main"},
                        (
                            input(attr! {type="checkbox", class="toggle-all", id="toggle-all"})
                                .on(Click, toggle_all),
                            label(attr! {for="toggle-all"}),
                            todo_list(records.clone()),
                        ),
                    )),
                    footer((
                        attr! {class="footer"},
                        span((
                            attr! {class="todo-count"},
                            (
                                // TODO use view here
                                strong(records.view(|data| data.val().items.len().into())),
                                span(" item(s) left"),
                                // TODO use view here
                                ul((attr! {class="filters"}, ())),
                                // TODO use view
                                button(attr! {class = "clear-completed"}),
                            ),
                        )),
                    )),
                ),
            )),
            footer((
                attr! {class="info"},
                (
                    p("Double-click to edit a todo"),
                    p((
                        span("Written by "),
                        a((
                            attr! {href="https://github.com/PrivateRookie", target="_blank"},
                            "PrivateRookie",
                        )),
                    )),
                    p((
                        span("Part of "),
                        a((
                            attr! {href="http://todomvc.com/", target="_blank"},
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
        if event.key() == "Enter" {
            if let Some(target) = event.target() {
                let input: HtmlInputElement = JsValue::from(target).into();
                let value = input.value();
                data.val_mut().add(value);
                input.set_value("");
            }
        }
    });
    input(attr! {id="edit-input",class="new-todo", placeholder="What needs to be done?"})
        .on(Keypress, update_and_reset)
}

fn todo_list(records: Reactive<Store>) -> Ul {
    ul(records.view(|data| {
        data.val()
            .items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                li((
                    div((
                        attr! {class="view"},
                        (
                            input(attr! {type="checkbox", class="toggle", checked=item.completed})
                                .on(
                                    Click,
                                    data.change(move |store| {
                                        store.val_mut().items.remove(idx);
                                        store.val_mut().save().unwrap();
                                    }),
                                ),
                            label(&item.description).on(
                                Click,
                                data.change(move |store| {
                                    if let Some(item) = store.val_mut().items.get_mut(idx) {
                                        item.editing = !item.editing;
                                    }
                                    store.val().save().unwrap();
                                }),
                            ),
                            button(attr! {class="destroy"}).on(
                                Click,
                                data.change(move |store| {
                                    store.val_mut().remove(idx);
                                }),
                            ),
                        ),
                    )),
                    item_edit_input(item, idx, data.clone()),
                ))
            })
            .collect::<Vec<_>>()
            .into()
    }))
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
        let on_mouseover = data.evt(|event, _| {
            if let Some(target) = event.target() {
                let input: HtmlInputElement = JsValue::from(target).into();
                input.focus().unwrap();
            }
        });

        let on_blur = data.evt(move |event, store| {
            edit(event, store, idx);
        });

        let on_keypress = data.evt(move |event, store| {
            let event: KeyboardEvent = JsValue::from(event).into();
            if event.key() == "Entry" {
                edit(event.into(), store, idx);
            }
        });
        input(attr! {class="edit", type="text", value=item.description})
            .on(Mouseover, on_mouseover)
            .on(Blur, on_blur)
            .on(Keypress, on_keypress)
    } else {
        input(attr! {type="hidden"})
    }
}
