use wasm_bindgen::prelude::*;

use rtml::{mount_body, tags::*, EventKind::*};

#[wasm_bindgen(start)]
pub fn start() {
    // 初始化 tracing
    tracing_wasm::set_as_global_default();

    // 不那么响应的响应式
    let count = 0usize.reactive();
    // 最外层组件
    let all_cards = rtml::tags::div((
        p(
            // 使用 view 获取数据试图, 这个视图会在 mutate 方法被调用后自动更新
            count.view(|times| {
                let s = if *times == 0 { "" } else { "s" };
                format!("count {times} time{s}").into()
            }),
        ),
        button("+1").on(
            Click,
            count.mutate(|data| {
                *data += 1;
            }),
        ),
        button("-1").on(
            Click,
            count.mutate(|data| {
                if *data > 0 {
                    *data -= 1;
                }
            }),
        ),
    ));
    // 渲染
    if let Err(e) = mount_body(all_cards) {
        tracing::error!("failed to init page, {:?}", e);
    }
}
