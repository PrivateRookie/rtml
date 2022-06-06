use rtml::{ref_subs, ref_update, tags::*};
use rtml::{
    t_attr,
    EventKind::{Blur, Click},
    IntoReactive, Reactive,
};
use wasm_bindgen::JsValue;
use web_sys::{Event, HtmlInputElement};

pub fn func_form() -> Form {
    let errors: Reactive<Vec<String>> = vec![].reactive();
    let name = String::new().reactive();
    let age = 0u8.reactive();

    let check_and_update_name = ref_update!(name, errors => |evt: Event| {
        let mut update = false;
        if let Some(target) = evt.target() {
            let input: HtmlInputElement = JsValue::from(target).into();
            let val = input.value();
            if val.len() > 10 {
                update = true;
                errors.val_mut().push("name is too long".into());
            } else {
                *name.val_mut() = val
            }
        }
        update
    });

    let check_and_update_age = ref_update!(age, errors => |evt: Event| {
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
    });

    let submit = ref_update!(name, age, errors => |evt: Event| {
        errors.val_mut().clear();
        if name.val().is_empty() {
            errors.val_mut().push("Name is required".into());
        }
        if *age.val() == 0 {
            errors.val_mut().push("age should > 0".into());
        }
        evt.prevent_default();
        true

    });
    let reset = ref_update!(name, age, errors => |_| {
        name.val_mut().clear();
        *age.val_mut() = 0;
        errors.val_mut().clear();
        true
    });
    form((
        t_attr!(id = "app"),
        (
            p(ul(
                ref_subs!(errors => errors.val().iter().map(li).collect::<Vec<_>>()),
            )),
            p((
                label((t_attr! (for="name"), "Name")),
                input((t_attr! ( id="name", type="text", name="name" ), ()))
                    .on(Blur, check_and_update_name)
                    .bind(ref_subs!(name :> |ele| {
                        let input: HtmlInputElement = JsValue::from(ele).into();
                            input.set_value(&name.val());
                    })),
            )),
            p((
                label((t_attr! {for="age"}, "Age")),
                input((
                    t_attr! { id="age", type="number", name="age", min="0", value="0" },
                    (),
                ))
                .bind(ref_subs!(age :> |ele| {
                    let input: HtmlInputElement = JsValue::from(ele).into();
                    input.set_value(&age.val().to_string());
                }))
                .on(Blur, check_and_update_age),
            )),
            p((
                label((t_attr! {for="movie"}, "Favorite Movie")),
                select((
                    t_attr! {id="movie", name="movie"},
                    (
                        option("Star Wars"),
                        option("Vanilla Sky"),
                        option("Atomic Blonde"),
                    ),
                )),
            )),
            p((
                input(t_attr! {type="submit", value="submit"}).on(Click, submit),
                button((t_attr! {type="button"}, "Reset")).on(Click, reset),
            )),
        ),
    ))
}
