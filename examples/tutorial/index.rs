use rtml::attr;
use rtml::tags::*;

#[rtml::page]
fn main() -> Html {
    let pkg = env!("CARGO_PKG_NAME").replace('-', "_");
    html((
        t_attr! {lang="zh-cn"},
        (
            head((
                meta(t_attr! { charset="utf-8" }),
                meta(t_attr! { http-equiv="Cache-Control", content="no-cache, no-store, must-revalidate" }),
                script((
                    t_attr! { type="module" },
                    format!("\nimport init from \"./{pkg}.js\";\ninit();\n"),
                )),
                title("RTML with WASM"),
            )),
            body(()),
        ),
    ))
}
