def_tag! {
  caption,
  Caption,
  CaptionArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<caption>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/caption)

---

**允许的内容**

<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content" title="HTML/Content categories#Flow content">Flow content</a>.

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

A <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> element, as its first descendant.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement)


---

## 属性

 包含全局属性: true

- align

	这个可枚举属性表明了 caption 相对于 table 应该如何排列。它可能有以下几个值：
 <ul>
  <li><code>left</code>, 展示在表格左边</li>
  <li><code>top</code>, 显示在表格前面</li>
  <li><code>right</code>, 显示在表格右边</li>
  <li><code>bottom</code>, 显示在表格下面</li>
 </ul>

 <div id="sect1" class="note notecard"><strong>使用说明：</strong>不要使用这个属性，它已经被弃用：&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/caption" aria-current="page"><code>&lt;caption&gt;</code></a> 元素应该使用&nbsp;<a title="CSS" href="/en-US/docs/Web/CSS">CSS</a>设置样式。要得到类似<code>align</code>属性的效果，使用&nbsp;<a href="/en-US/docs/Web/CSS" title="CSS">CSS</a> 属性 <a href="/zh-CN/docs/Web/CSS/caption-side"><code>caption-side</code></a> 和 <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a>.</div>
 "#####
}
