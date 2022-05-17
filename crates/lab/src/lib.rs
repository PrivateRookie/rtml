use wasm_bindgen::prelude::*;

use rtml::{mount_body, tags::*, EventKind::*};

#[wasm_bindgen(start)]
pub fn start() {
    // 初始化 tracing
    tracing_wasm::set_as_global_default();
    let count = 0usize.reactive();
    let other = count.clone();
    // 最外层组件
    let all_cards = rtml::tags::div((
        p(count.view(|times| format!("count {times} time(s)").into())),
        button("+1").when(
            Click,
            count.mutate(|data| {
                *data += 1;
            }),
        ),
        button("-1").when(
            Click,
            other.mutate(|data| {
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
