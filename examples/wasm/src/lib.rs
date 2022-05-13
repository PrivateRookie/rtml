use wasm_bindgen::prelude::*;

use rtml::{attr, mount_body, style, tags::*, EventKind};

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();
    let all_cards = rtml::tags::div((
        // 调用其他组件
        count_card("card1".to_string(), Some(100), None),
        count_card("card2".to_string(), Some(20), None),
    ));
    // 渲染
    if let Err(e) = mount_body(all_cards) {
        tracing::error!("failed to init page, {:?}", e);
    }
}

// 某个块可以方便地抽成组件, 组件初始化参数即为函数参数
fn count_card(desc: String, init: Option<usize>, btn_label: Option<String>) -> rtml::tags::Div {
    let init = init.unwrap_or_default();
    // 根据初始值设置显示, 注意只是设置 <p> 内容, 不会将 init 和 <p> 绑定
    let show = p(init);
    let incr = button(btn_label.unwrap_or_else(|| "click".to_string()))
        // 将 init 和 <button>绑定
        .inject(init)
        // 将 <button> 和 <p> 连接到一起, 方便以后的事件处理
        .link(show.mark())
        // 添加点击事件处理函数
        // 传入的匿名函数第一个参数总是指向正在创建的 ele,
        // 根据 之前 link 参数不同, 其他参数个数也会相应变化
        .on(EventKind::Click, |(btn, show)| {
            Box::new(move || {
                tracing::info!("clicked");
                // 从 btn 拿到从 take 函数中传入的数据
                let mut count = btn.data.borrow_mut();
                // 更新数据
                *count += 1;
                // 从 show 拿到 button 对应的 html 元素, 并进行更新操作
                // rtml 还没做好响应式设计
                let e = show.ele.borrow();
                let e = e.as_ref().unwrap();
                e.set_inner_html(&count.to_string());
            })
        });

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
