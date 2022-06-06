use rtml::{t_attr, tags::*};
use rtml_project::{add_file, get_pkg_id};

fn index() -> Html {
    let pkg = get_pkg_id();
    html((
        t_attr! {lang="zh-cn"},
        (
            head((
                meta(t_attr! { charset="utf-8" }),
                meta(t_attr! { name = "viewport", content="width=device-width, initial-scale=1" }),
                meta(
                    t_attr! { http-equiv="Cache-Control", content="no-cache, no-store, must-revalidate" },
                ),
                title(&pkg),
                script((
                    t_attr! { type="module" },
                    format!("\nimport init from \"./{pkg}.js\";\ninit();\n"),
                )),
            )),
            body(()),
        ),
    ))
}

fn main() -> std::io::Result<()> {
    add_file(
        "index.html",
        index().to_string().as_bytes(),
    )
}
