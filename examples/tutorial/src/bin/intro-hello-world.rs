use rtml::tags::{h1, html, Html};

#[rtml::page]
fn adding_data() -> Html {
    let name = "world";
    html((
        // simple format macro to capture data
        h1(format!("hello {name}!")),
        h1(format!("hello {}", name.to_uppercase())),
    ))
}
