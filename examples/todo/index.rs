use rtml::{attr, tags::*};

#[rtml::page]
fn index() -> Html {
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
                // title("RTML • TodoMVC"),
                title("RTML • TodoMVC"),
                script((
                    attr! { type="module" },
                    format!("\nimport init from \"./{pkg}.js\";\ninit();\n"),
                )),
                style(include_str!(
                    "assets/base.css"
                )),
                style(include_str!(
                    "assets/index.css"
                )),
            )),
            body(()),
        ),
    ))
}
