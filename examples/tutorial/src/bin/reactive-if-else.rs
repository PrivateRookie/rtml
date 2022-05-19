use rtml::attr;
use rtml::tags::*;

#[rtml::page]
fn reactive_if_else() -> Html {
    let pkg = env!("CARGO_PKG_NAME").replace('-', "_");
    let bin = env!("CARGO_BIN_NAME").replace('-', "_");
    html((
        attr! {lang="zh-cn"},
        (
            head((
                meta(attr! { charset="utf-8" }),
                meta(attr! { http-equiv="Cache-Control", content="no-cache, no-store, must-revalidate" }),
                script((
                    attr! { type="module" },
                    format!(r#"
                    import init from "./{pkg}.js";
                    import {{ {bin} }} from "./{pkg}.js";
                    init().then(() => {bin}());
                    "#),
                )),
                title("Tutorial - RTML"),
            )),
            body(()),
        ),
    ))
}
