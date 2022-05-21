use rtml::{attr, tags::*};
#[rtml::page]
fn main() -> Html {
    let pkg = env!("CARGO_PKG_NAME").replace('-', "_");
    html((
        attr! {lang="zh-cn"},
        (
            head((
                meta(attr! { charset="utf-8" }),
                meta(attr! { name = "viewport", content="width=device-width, initial-scale=1" }),
                meta(
                    attr! { http-equiv="Cache-Control", content="no-cache, no-store, must-revalidate" },
                ),
                meta(attr! { author = "Dcy"}),
                script((
                    attr! { type="module" },
                    format!("\nimport init from \"./{pkg}.js\";\ninit();\n"),
                )),
                style(include_str!("./assets/style.css")),
                link(attr!{rel="stylesheet", href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.8.2/font/bootstrap-icons.css"}),
                title("ToDoMVC"),
            )),
            body(()),
        ),
    ))
}
