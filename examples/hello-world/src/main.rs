use rtml::attr;
use rtml::tags::*;

#[rtml::page]
// 函数返回值必须为 Html, 即 html 根元素
fn main() -> Html {
    // 使用与 html 标签同名函数构造对应的 html 元素
    html((
        // attr! 用于设置 html 元素属性, 这这里设置一个最常见 lang 属性
        attr! { lang="zh-cn" },
        // 属性之后可跟着样式设置(可选)和子元素, 这里忽略样式设置
        (
            head((
                // 对于 meta 等标签, 我们不关心内容, 只传入属性也是合法的参数
                meta(attr! { charset="utf-8" }),
                // 直接传入字符串, 浮点数, 整数, 布尔值会被自动设置为 html 元素文字内容
                title("RTML Siple Template"),
            )),
            body((h1("hello world!"), h1("你好, 世界!"))),
        ),
    ))
}
