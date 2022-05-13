use rtml::attr;
use rtml::tags::*;

#[rtml::page]
fn main() -> Html {
    let pkg = env!("CARGO_PKG_NAME").replace("-", "_");
    html((
        attr! {lang="zh-cn"},
        (
            head((
                meta(attr! { charset="utf-8" }),
                script((
                    attr! { type="module" },
                    format!("\nimport init from \"./{pkg}.js\";\ninit();\n"),
                )),
                title("RTML with WASM"),
            )),
            body(()),
        ),
    ))
}
