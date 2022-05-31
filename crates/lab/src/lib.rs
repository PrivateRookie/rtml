use rtml::{attr, mount_body, tags::*, debug_auto_reload};
use wasm_bindgen::prelude::*;

fn my_form() -> Form {
    let f = form((
        attr! {
            id="app",
            action="https://vuejs.org/",
            method="post"
        },
        (
            p((
                label((attr! {for="name"}, "Name")),
                input((
                    attr! {
                        id="name",
                        type="text",
                        name="name"
                    },
                    (),
                )),
            )),
            p((
                label((attr! {for="age"}, "Age")),
                input((
                    attr! {
                        id="age",
                        type="number",
                        name="age",
                        min="0"
                    },
                    (),
                )),
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
            p(input(
                attr! {type="submit", value="submit"},
            )),
        ),
    ));
    f
}

#[wasm_bindgen(start)]
pub fn start() {
    debug_auto_reload();
    mount_body(my_form()).unwrap();
}
