use rtml::tags::{html, p, style, Html};

fn styling() -> Html {
    html((
        p("This is a paragraph."),
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
    println!("{}", styling());
}
