use rtml::tags::{h1, html, Html};

#[rtml::page]
fn hello_world() -> Html {
    html(h1("hello world!"))
}
