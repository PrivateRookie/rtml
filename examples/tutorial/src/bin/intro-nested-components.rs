use rtml::tags::{html, p, style, Html, P};

// component is just a normal function
fn nested() -> P {
    p("This is another paragraph.")
}

fn nested_components() -> Html {
    html((
        p("This is a paragraph."),
        nested(),
        style(
            r#"
        p {
            color: purple;
            font-family: 'Comic Sans MS', cursive;
            font-size: 2em;
        }
        "#,
        ),
    ))
}

fn main() {
    println!("{}", nested_components());
}
