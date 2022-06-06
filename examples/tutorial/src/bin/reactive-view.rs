use rtml::t_attr;
use rtml::tags::*;

fn reactive_view() -> Html {
    let pkg = env!("CARGO_PKG_NAME").replace('-', "_");
    let bin = env!("CARGO_BIN_NAME").replace('-', "_");
    html((
        t_attr! {lang="zh-cn"},
        (
            head((
                meta(t_attr! { charset="utf-8" }),
                meta(
                    t_attr! { http-equiv="Cache-Control", content="no-cache, no-store, must-revalidate" },
                ),
                script((
                    t_attr! { type="module" },
                    format!(
                        r#"
                    import init from "./{pkg}.js";
                    import {{ {bin} }} from "./{pkg}.js";
                    init().then(() => {bin}());
                    "#
                    ),
                )),
                title("Tutorial - RTML"),
            )),
            body(()),
        ),
    ))
}

fn main() {
    println!("{}", reactive_view());
}
