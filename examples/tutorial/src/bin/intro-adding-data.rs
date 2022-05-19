use rtml::tags::{h1, html, Html};

#[rtml::page]
fn adding_data() -> Html {
    html(h1("hello world!"))
}
