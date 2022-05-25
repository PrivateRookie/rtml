def! {
  param,
  Param,
  ParamArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<param>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/param)

---

**允许的内容**

不允许，它是一个空元素（<a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>）。

**是否可忽略关闭标签**

由于它是一个 void 元素，所以开始标签必须出现，而结束标签必须不出现。

**父标签**

任何<a href="/zh-CN/docs/Web/Guide/HTML/Content_categories#flow_content#Flow_content">以下内容</a>（<a href="/en-US/docs/HTML/Content_categories#Flow_content">flow content</a>）都可以在<a href="/zh-CN/docs/Web/HTML/Element/object"><code>&lt;object&gt;</code></a>元素的前面作为它的父元素。

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement)


---

## 属性

 包含全局属性: true

- name

	参数的名字。
- type

	仅当 valuetype 设置为 “ref” 时才使用。根据 URI 中给定的数据确定 MIME 类型。
- value

	确定参数的值。
- valuetype

	确定参数的类型。可选值如下：
 <ul>
  <li>data: 默认值。该值作为字符串变量传递给对象实例。</li>
  <li>ref: 该值是存储运行时变量的资源的 URI。</li>
  <li>object: 同一页面（document）中另一个<a href="/zh-CN/docs/Web/HTML/Element/object"><code>&lt;object&gt;</code></a>的 ID。</li>
 </ul>
 "#####
}
