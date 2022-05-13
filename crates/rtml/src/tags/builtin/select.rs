def! {
  select,
  Select,
  SelectArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<select>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/select)

---

**允许的内容**

Zero or more <a href="/zh-CN/docs/Web/HTML/Element/option"><code>&lt;option&gt;</code></a> or <a href="/zh-CN/docs/Web/HTML/Element/optgroup"><code>&lt;optgroup&gt;</code></a> elements.

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

Any element that accepts&nbsp;<a title="Currently only available in English (US)" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content" class="only-in-en-us">phrasing content (en-US)</a>.

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLSelectElement)


---

## 属性

 包含全局属性: true

- autocomplete

	一个 <a href="/zh-CN/docs/Web/API/DOMString"><code>DOMString</code></a>，为 <a href="/zh-CN/docs/Glossary/User_agent">用户代理</a> 提供自动填充功能的线索。 关于该值的完整列表以及如何使用自动填充的详细信息，请参阅&nbsp;<a href="/zh-CN/docs/Web/HTML/Attributes/autocomplete">HTML 自动完成属性</a>。
- autofocus

	这个布尔值属性能够让一个对象在页面加载的时候获得焦点。一个文档中只有一个对象可以有这个属性。
- disabled

	这个布尔值的属性表示用户不能与该表单控件交互。如果没有声明这个属性，则从它的父元素继承，例如&nbsp;<code>fieldset</code>；如果没有父元素设置了&nbsp;<code>disabled</code> 属性，那么默认该表单对象可用。
- form

	&nbsp;<code>&lt;select&gt;</code>&nbsp;所关联的<a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> (它的"表单拥有者")。其值必须是在同一文档中的 <code>&lt;form&gt;</code>&nbsp;元素的<a href="/zh-CN/docs/Web/HTML/Global_attributes#attr-id"><code>id</code></a>（如果没有设置这个属性，&nbsp;<code>&lt;select&gt;</code>&nbsp;元素则与其任何存在的祖先 <code>&lt;form&gt;</code>&nbsp;元素关联）。
- multiple

	这个布尔值属性表示列表中的选项是否支持多选。没有声明该值时，一次只能选中一个选项。声明这个属性后，大多数浏览器都会显示一个可滚动的列表框，而非一个下拉菜单。
- name

	该属性规定了控件的名称。
- required

	一个布尔值属性，表示必须选中一个有非空字符串值的选项。
- size

	如果控件显示为滚动列表框（如声明了 <code>multiple</code>），则此属性表示为控件中同时可见的行数。浏览器不需要将选择元素呈现为滚动列表框。默认值为 0。"#####
}

