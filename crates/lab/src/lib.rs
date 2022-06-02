use rtml::{
    add, attr, mount_body,
    tags::*,
    EventKind::{Blur, Click},
    IntoReactive, Reactive,
};
use rtml_project::debug_auto_reload;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlInputElement};

fn my_form() -> Form {
    let errors: Reactive<Vec<String>> = vec![].reactive();
    let name = String::new().reactive();
    let age = 0u8.reactive();

    let x = rtml::view!(name, age => {
        format!("{} is {} old", name.val(), age.val())
    });

    let f = form((
        attr! {
            id="app",
            action="https://vuejs.org/",
            method="post"
        },
        (
            p(ul(
                rtml::view!(errors => errors.val().iter().map(li).collect::<Vec<_>>()),
            )),
            p((
                label((attr! {for="name"}, "Name")),
                input((
                    attr! {
                        id="name",
                        type="text",
                        name="name"
                    },
                    (),
                ))
                .on(
                    Blur,
                    rtml::update!(name, errors => |evt: Event| {
                        let mut update = false;
                        if let Some(target) = evt.target() {
                            let input: HtmlInputElement = JsValue::from(target).into();
                            let val = input.value();
                            if val.len() > 10 {
                                update = true;
                                errors.val_mut().push("name is too long".into());
                            } else {
                                *name.val_mut() = val.to_string()
                            }
                        }
                        update
                    }),
                ),
            )),
            p((
                label((attr! {for="age"}, "Age")),
                input((
                    attr! {
                        id="age",
                        type="number",
                        name="age",
                        min="0",
                        value="0"
                    },
                    (),
                ))
                .on(
                    Blur,
                    rtml::update!(age, errors => |evt: Event| {
                        let mut update = false;
                        if let Some(target) = evt.target() {
                            let input: HtmlInputElement = JsValue::from(target).into();
                            match input.value().parse::<u8>() {
                                Ok(val) => {
                                    *age.val_mut() = val;
                                }
                                Err(e) => {
                                    errors.val_mut().push(e.to_string());
                                    update = true;
                                }
                            }
                        }
                        update
                    }),
                ),
            )),
            p((
                label((
                    attr! {for="movie"},
                    "Favorite Movie",
                )),
                select((
                    attr! {id="movie", name="movie"},
                    (
                        option("Star Wars"),
                        option("Vanilla Sky"),
                        option("Atomic Blonde"),
                    ),
                )),
            )),
            p(
                input(attr! {type="submit", value="submit"}).on(
                    Click,
                    rtml::update!(name, age, errors => |evt: Event| {
                        errors.val_mut().clear();
                        if name.val().is_empty() {
                            errors.val_mut().push("Name is required".into());
                        }
                        if *age.val() == 0 {
                            errors.val_mut().push("age should > 0".into());
                        }
                        evt.prevent_default();
                        true

                    }),
                ),
            ),
        ),
    ));
    f
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();
    debug_auto_reload();
    mount_body(my_form()).unwrap();
}
