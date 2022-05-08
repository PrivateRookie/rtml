use rtml::prop;
use rtml::tags::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let page = html((
        prop! { lang="zh-ch" },
        (
            head((
                meta(prop! { charset="utf-8" }),
                script((
                    prop! { type="module" },
                    format!(
                        r#"import init from "./{}.js"; init();"#,
                        env!("CARGO_PKG_NAME").replace("-", "_")
                    ),
                )),
                title("RTML with WASM"),
            )),
            body(()),
        ),
    ));
    let content = page.to_string();
    File::create("crates/wasm/pkg/index.html")
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}
