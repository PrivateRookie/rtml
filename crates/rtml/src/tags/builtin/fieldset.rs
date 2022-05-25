def! {
  fieldset,
  Fieldset,
  FieldsetArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<fieldset>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/fieldset)

---

**允许的内容**

可选的<a href="/zh-CN/docs/Web/HTML/Element/legend"><code>&lt;legend&gt;</code></a> 元素，后面是内容流（flow content）

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

Any element that accepts&nbsp;<a title="Currently only available in English (US)" href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content" class="only-in-en-us">flow content (en-US)</a>.

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLFieldSetElement)


---

## 属性

 包含全局属性: true

- disabled

	如果设置了这个 bool 值属性，<code>&lt;fieldset&gt;</code>&nbsp;的所有子代表单控件也会继承这个属性。这意味着它们不可编辑，也不会随着 <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> 一起提交。它们也不会接收到任何浏览器事件，如鼠标点击或与聚焦相关的事件。默认情况下，浏览器会将这样的控件展示为灰色。注意，<a href="/zh-CN/docs/Web/HTML/Element/legend"><code>&lt;legend&gt;</code></a>&nbsp;中的表单元素不会被禁用。
- form

	将该值设为一个 <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> 元素的 <a href="/zh-CN/docs/Web/HTML/Global_attributes#attr-id"><code>id</code></a> 属性值以将 <code>&lt;fieldset&gt;</code>&nbsp;设置成这个 <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> 的一部分。&nbsp; &nbsp; &nbsp;&nbsp;
- name

	元素分组的名称
 <div class="note notecard" id="sect5"><strong>注意：</strong>fieldset 的标题由第一个 <a href="/zh-CN/docs/Web/HTML/Element/legend"><code>&lt;legend&gt;</code></a> 子元素确定。</div>
 "#####
}
