use rtml::{button, form, input, label, option, p, select, IntoReactive, Reactive};
use wasm_bindgen::JsValue;
use web_sys::{Event, HtmlInputElement};

pub fn macro_form() -> rtml::tags::Form {
    let errors: Reactive<Vec<String>> = vec![].reactive();
    let name = String::new().reactive();
    let age = 0u8.reactive();
    form! {
        #{id="app"}
        || (
            p! {|errors| errors.val().iter().map(rtml::tags::li).collect::<Vec<_>>()},
            p! {|| (
                label! {#{for="name"} || "Name"},
                input!(
                    # {id="name" type="text" name="name"}
                    :name => |ele| {
                        let input: HtmlInputElement = JsValue::from(ele).into();
                        input.set_value(&name.val());
                    };
                    @blur = name, errors => |evt: Event| {
                        let mut update = false;
                        if let Some(target) = evt.target() {
                            let input: HtmlInputElement = JsValue::from(target).into();
                            let val = input.value();
                            if val.len() > 10 {
                                update = true;
                                errors.val_mut().push("name is too long".into());
                            }
                            *name.val_mut() = val;
                        }
                        update
                    };
                )
            )},
            p! {|| (
                label! {#{for="age"} || "Age"},
                input!(
                    # {id="age" type="number" name="age" min="0" value="0"}
                    : age => |ele| {
                        let input: HtmlInputElement = JsValue::from(ele).into();
                        input.set_value(&age.val().to_string());
                    };
                    @blur = age, errors => |evt: Event| {
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
                    };
                )
            )},
            p!{|| (
                label!{#{for="movie"} || "Favorite Movie"},
                select!(#{id="movie" name="movie"} || (
                    option!(|| "Star Wars"),
                    option!(|| "Vanilla Sky"),
                    option!(|| "Atomic Blonde"),
                ))
            )},
            p! {|| (
                input! {
                    #{type="submit" value="submit"}
                    @click = name, age, errors => |evt: Event| {
                        errors.val_mut().clear();
                        if name.val().is_empty() {
                            errors.val_mut().push("Name is required".into());
                        }
                        if *age.val() == 0 {
                            errors.val_mut().push("age should > 0".into());
                        }
                        evt.prevent_default();
                        true
                    };
                },
                button! {
                    #{type="button"}
                    @click = name, age, errors => |_| {
                        name.val_mut().clear();
                        *age.val_mut() = 0;
                        errors.val_mut().clear();
                        true
                    };
                    || "Reset"
                }
            )},

        )
    }
}
