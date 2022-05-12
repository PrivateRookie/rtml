def! {
  label,
  Label,
  LabelArg,
  doc:
  "zh-CN" = r#####"`<label>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/label)

---

**允许的内容**

<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>, but no descendant <code>label</code> elements. No labelable elements other than the labeled control are allowed.

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>.

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLLabelElement)


---

## 属性

 包含全局属性: true

- for

	即和&nbsp;<code>&lt;label&gt;</code>&nbsp;元素在同一文档中的&nbsp;<a href="zh-CN/docs/Web/Guide/HTML/Content_categories#Form_labelable">可关联标签的元素</a>&nbsp;的 <a href="/zh-CN/docs/Web/HTML/Global_attributes#attr-id"><code>id</code></a>。 文档中第一个&nbsp;<code>id</code>&nbsp;值与&nbsp;<code>&lt;label&gt;</code>&nbsp;元素&nbsp;<code>for</code>&nbsp;属性值相同的元素，如果可关联标签（labelable），则为<em>已关联标签的控件</em>，其标签就是这个&nbsp;<code>&lt;label&gt;</code>&nbsp;元素。如果这个元素不可关联标签，则&nbsp;<code>for</code>&nbsp;属性没有效果。如果文档中还有其他元素的&nbsp;<code>id</code>&nbsp;值也和&nbsp;<code>for</code>&nbsp;属性相同，<code>for</code>&nbsp;属性对这些元素也没有影响。
 <div id="sect4" class="note notecard"><strong>注意：</strong><code>&lt;label&gt;</code>&nbsp;元素可同时有一个&nbsp;<code>for</code>&nbsp;属性和一个子代控件元素，只是&nbsp;<code>for</code>&nbsp;属性需要指向这个控件元素。</div>
 
- form

	表示与 label 元素关联的&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> 元素（即它的表单拥有者）。如果声明了该属性，其值应是同一文档中 <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> 元素的 <code>id</code>。因此你可以将 label 元素放在文档的任何位置，而不仅作为&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> 元素的后代。"#####
}

