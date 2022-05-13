# rtml

以 Rust 风格编写网页.

## 入门

rtml 是一个让你以 Rust 代码实现编写网页的库, 换言之, rtml 让用户可以操作 html/css/js.


以最简单的 hello world 为例, `cargo new` 新建一个项目, 在添加 rtml 依赖, 在 main.rs 中添加如下代码

```rust
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
                title("RTML Simple Template"),
            )),

            body((
                h1("hello world!"),
                h1("你好, 世界!")
            )),
        ),
    ))
}
```

rtml::page 是一个方便用户使用的过程宏, 它的作用不过是将被修饰函数返回值作为整个 html 页面模板, 打印并保存到 CARGO_MANIFEST_DIR 目录下的 pkg 目录(目录名可改, 见 rtml::page 宏介绍). 真正有用的是我们在 main 函数中通过调用与 html 标签同名的函数创建标签, 在 Rust 代码中尽量与直接书写 html 的风格接近.

如 body() 类似于 \<body\>\</body\>, 只不过 rtml 省略了一次标签名, 同时用小括号代替了开闭元素. 此外先属性后子元素的参数位置要求, 也于 html 中的 \<div id="app"\>xxxx\</div\> 对应.

接着我们通过 cargo run 运行这个项目, 运行成功后会在项目 pkg 目录下生成 index.html(文件名可改, 见 rtml::page 宏介绍) 文件.

```html
<html lang="zh-cn">
    <head>
        <meta charset="utf-8"></meta>
        <title>RTML Siple Template</title>
</head>
    <body>
        <h1>hello world!</h1>
        <h1>你好, 世界!</h1>
</body>
</html>
```

可以看到我们传递的 html 标签和属性都被正确渲染了, 用浏览器打开即可看到网页效果. 这个例子可以在 [examples/hello-world](./examples/hello-world/) 找到.

### 样式设置

和在 html 中设置标签样式类似, 有两种方法

1. 用户可以在单个标签上通过 style 属性设置
2. 通过 link 或 style 标签, 引入样式文件或嵌入 css 代码

对于第一种方式, rtml 提供了 style! 将其作为标签构造参数传入标签同名函数.

```rust
div((
    style! { color: "red" },
    p("should be red text")
))
```

值得注意的是 rtml 标签参数, 样式和内容的传参方式非常灵活, 你可以只传任意一个, 下面三个 div 函数实参都是合法的

```rust
div((
    div("content only"),
    // attribute only
    div(attr! { id = "app" }),
    // style only
    div(style! { color: "red" })
))
```

如果需要同时传入属性,样式和内容, 除了要求有内容时, 内容必须放在最后位置外, 对属性和样式顺序无要求.

```rust
div((
    div((attr! {}, "attr and then content, OK")),
    div((style! {}, "style and then content, OK")),
    div((attr! {}, style! {}, "attr then style and then content, OK")),
    div((style! {}, attr! {}, "order don't matter for attr and style")),
    div((style! {}, attr! {}, "order don't matter for attr and style")),
    // no content is also ok
    div((style! {}, attr! {})),
))
```

对于第二种方式, 在需要地方构建 style 标签即可, 比如你想通过 CDN 引入 zui 的 css 库

```rust
head((
    link(attr! {
        rel = "stylesheet",
        href = "https://cdn.bootcdn.net/ajax/libs/zui/1.10.0/css/zui.min.css"
    }),
))
```

### 标签内容

同 html 标签一样, 一个标签通常可以包含字符或子标签, 但因为 Rust 是静态强类型语言, 标签同名函数可以当做内容的类型有如下几种.

#### 基本字面类型

String, &str, bool, u8~u64, i8~i64, f32, f64 和 ()

```rust
// empty content
p(())

p("abc")
p(format!("hello, {}", "rookie"))
p(true)
p(100u64)
```

由此带来的便利是, 你只需要记住 format! 格式就能往标签内容写入某些特定格式内容, 不需要再额外学习模板语法

#### 单个 html 标签

如

```rust
div(h1("Great!"))
```

#### 多个不同类型标签

一个 div 有 p, h2, span 等不同类型子标签是 html 中的常态, 如果子元素个数不是很多, 可以将他们作为一个元组传入

```rust
div((
    h2("Something Big"),
    hr(()),
    p("more details")
))
```

rtml 中元组长度最多为 34, 如果需要传入更多, 请使用 vec! 宏并将这些元素通过 Box 包一层.

需要注意的是, 虽然 str, u8 等可以作为元素内容单独传入, 且在 html 中以下写法是正确的

```html
<div>
    isolate
    <p>wrapped</p>
</div>
```

但在 rtml 里他们不能与其他标签混用, 即

```rust
div((
    // error
    "isolate",
    p("wrapped")
))
```

这样的用法是错误的, 无法通过编译, 你必须将这些文字包在某个标签中, 如 span.


#### 多个同类型元素

rtml 支持传入 Vec\<T\> 作为内容, 配合 Rust 的 collect 方法, 你可以在 rtml 里方便地写出循环生成标签.

```rust
let items = vec![1, 2, 3, 4];
let many_div = div(items
    .iter()
    .map(|i| div(format!("item {i}")))
    .collect::<Vec<_>>());
```

### 处理事件和相关数据

在之前的介绍里已经展示了 rtml 如何处理 html/css, 但只有这些还不够, 我们希望 rtml 能以某种方式处理 js, 让页面能动起来.
最简单也是最朴素的方法是类似 css 那样, 通过 link 或 script 引入 js 文件直接以字符串的形式在 script 中写 js.

但这无疑会带来极差的开发体验, 在写 rust 代码时我们可以充分利用 Rust 语言特性和 LSP 带来的辅助功能, 像驾驶自动挡汽车一样轻松地书写安全的代码, 但在写嵌入的 JS 代码时连基本的代码高亮和补全都没有, 仿佛一下回到上古世纪, 这绝对不行!

万幸的是 Rust wasm 支持最好的语言, 可以想办法在 Rust 中写好事件处理代码, 然后将他们编译成 wasm 文件, 最后在生成的 html 文件中 link 这些文件即可.
