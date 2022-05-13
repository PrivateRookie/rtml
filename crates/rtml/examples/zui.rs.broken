use std::{fs::File, io::Write};

use rtml::{prop, style, tags::*, Children};

fn main() {
    let page = webpage((
        p((
            "power by ",
            a((prop! { href="https://www.openzui.com/#/" }, "ZUI")),
        )),
        h1("icons"),
        div(ul((
            li((i(prop! { class = "icon-flag" }), "普通图标")),
            li((i(prop! { class = "icon-heart"}), "icon-heart")),
            li((i(prop! { class = "icon icon-flag" }), "等宽图表")),
            li((
                i(prop! { class = "icon icon-heart icon-4x", big }),
                "icon-big-heart",
            )),
        ))),
        hr(()),
        h1("progress bar"),
        div((
            prop! { class = "progress" },
            div((
                prop! {
                    class="progress-bar progress-bar-success",
                    role="progressbar",
                    aria-valuenow="40",
                    aria-valuemin="0",
                    aria-valuemax="100",
                    title="bar"
                },
                style! {
                    width: "40%"
                },
                span((prop! { class="sr-only" }, "40% complete")),
            )),
        )),
    ));
    let page = page.to_string();
    save_and_open(&page, "target/index.html");
}

fn webpage<B: Into<Children>>(content: B) -> Html {
    let mut children = content.into();

    children.push(script(prop! {
        src = "https://cdn.bootcdn.net/ajax/libs/zui/1.10.0/lib/jquery/jquery.js"
    }));
    children.push(script(prop! {
        src = "https://cdn.bootcdn.net/ajax/libs/zui/1.10.0/js/zui.min.js"
    }));

    html((
        prop! { lang = "zh-cn" },
        (
            head((
                meta(prop! { charset = "utf-8" }),
                meta(prop! { name = "viewport", content="width=device-width, initial-scale=1" }),
                link(prop! {
                    rel = "stylesheet",
                    href = "https://cdn.bootcdn.net/ajax/libs/zui/1.10.0/css/zui.min.css"
                }),
                title("Rtml with Zui"),
            )),
            body(children),
        ),
    ))
}

fn save_and_open(content: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    opener::open(path).unwrap()
}
