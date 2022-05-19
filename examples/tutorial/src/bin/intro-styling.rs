use rtml::tags::{html, p, style, Html};

#[rtml::page]
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
