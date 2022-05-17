use wasm_bindgen::prelude::*;

use rtml::{attr, mount_body, style, tags::*, EventKind::*};

#[wasm_bindgen(start)]
pub fn start() {
    // 初始化 tracing
    tracing_wasm::set_as_global_default();
    // 最外层组件
    let all_cards = rtml::tags::div((
        // 调用其他组件
        count_card("card1", 100, None),
        count_card("card2", 20, "+1s"),
    ));
    // 渲染
    if let Err(e) = mount_body(all_cards) {
        tracing::error!("failed to init page, {:?}", e);
    }
}

// 某个块可以方便地抽成组件, 组件初始化参数即为函数参数
fn count_card<I: Into<Option<usize>>, B: Into<Option<&'static str>>>(
    desc: &str,
    init: I,
    btn_label: B,
) -> rtml::tags::Div {
    let count = init.into().unwrap_or_default().reactive();
    // 根据初始值设置显示, 注意只是设置 <p> 内容, 不会将 init 和 <p> 绑定
    let show = p(count.view(|data| data.to_string().into()));
    let label = btn_label.into().unwrap_or("click");
    let incr = button(label).on(
        Click,
        count.mutate(|data| {
            *data += 1;
        }),
    );

    div((
        attr! {
            class="count-card"
        },
        style! {
            border: "black solid 1px";
            padding: "10px";
            margin: "10px 0 10px 0"
        },
        (h2(desc), show, incr),
    ))
}
