def! {
  style,
  Style,
  StyleArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<style>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/style)

---

**允许的内容**

<dfn>允许的内容</dfn>与&nbsp;<code>type</code> 属性相匹配的文本内容，也就是&nbsp;<code>text/css</code>

**是否可忽略关闭标签**

<dfn>标签忽略</dfn> 不允许，开始标签和结束标签都不能省略。

**父标签**

<dfn>允许的父元素</dfn> 任意接受<a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/Guide/HTML/Content_categories#metadata_content">元数据内容 (en-US)</a>的元素

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLStyleElement)


---

## 属性

 包含全局属性: true

- media

	该属性规定该样式适用于哪个媒体。属性的取值<a href="/zh-CN/docs/Web/CSS/Media_Queries/Using_media_queries">CSS 媒体查询</a>，默认值为 <code>all</code>。
- nonce

	一种加密的随机数（一次使用的数字），用于在<a href="/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/style-src" title="Currently only available in English (US)" class="only-in-en-us">style-src Content-Security-Policy (en-US)</a>中将内联样式列入白名单。 服务器每次发送策略时都必须生成一个唯一的随机数值。 提供一个无法猜测的随机数非常重要，因为绕开资源策略是微不足道的。
- title

	指定可选的样式表。
- type

	该属性以 MIME 类型（不应该指定字符集）定义样式语言。如果该属性未指定，则默认为 <code>text/css</code>。"#####
}
