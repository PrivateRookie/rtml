use rtml::attr;
use rtml::tags::*;
use rtml::Marker;

#[rtml_macro::page]
fn main() -> Html<Marker> {
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
