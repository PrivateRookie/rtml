use rtml::{attr, tags::*};

#[rtml::page]
fn main() -> Html {
    let pkg = env!("CARGO_PKG_NAME").replace("-", "_");
    html((
        attr! {lang="zh-cn"},
        (
            head((
                meta(attr! { charset="utf-8" }),
                meta(attr! { name = "viewport", content="width=device-width, initial-scale=1" }),
                meta(
                    attr! { http-equiv="Cache-Control", content="no-cache, no-store, must-revalidate" },
                ),
                link(attr! {
                    rel = "stylesheet",
                    href = "https://cdn.jsdelivr.net/npm/bootstrap@5.2.0-beta1/dist/css/bootstrap.min.css",
                    integrity="sha384-0evHe/X+R7YkIZDRvuzKMRqM+OrBnVFBL6DOitfPri4tjfHxaWutUpFmBp4vmVor",
                    crossorigin="anonymous"
                }),
                link(attr! {
                    rel="stylesheet",
                    href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.8.2/font/bootstrap-icons.css"
                }),
                script(attr! {
                    src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.0-beta1/dist/js/bootstrap.bundle.min.js",
                    integrity="sha384-pprn3073KE6tl6bjs2QrFaJGz5/SUsLqktiwsUTF55Jfv3qYSDhgCecCxMW52nD2",
                    crossorigin="anonymous"
                }),
                script((
                    attr! { type="module" },
                    format!("\nimport init from \"./{pkg}.js\";\ninit();\n"),
                )),
                title("TODO"),
            )),
            body(()),
        ),
    ))
}
