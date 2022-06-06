def_tag! {
  menu,
  Menu,
  MenuArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<menu>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/menu)

---

**允许的内容**

If the element is in the <em>list menu</em> state: <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>, or alternatively, zero or more occurrences of <a href="/zh-CN/docs/Web/HTML/Element/li"><code>&lt;li&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/script"><code>&lt;script&gt;</code></a>, and <a href="/zh-CN/docs/Web/HTML/Element/template"><code>&lt;template&gt;</code></a>.<br>
	If the element is in the <em>context menu</em> state: zero or more occurrences, in any order, of <a href="/zh-CN/docs/Web/HTML/Element/menu" aria-current="page"><code>&lt;menu&gt;</code></a> (<em>context menu</em> state only), <a href="/zh-CN/docs/Web/HTML/Element/menuitem"><code>&lt;menuitem&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/hr"><code>&lt;hr&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/script"><code>&lt;script&gt;</code></a>, and <a href="/zh-CN/docs/Web/HTML/Element/template"><code>&lt;template&gt;</code></a>.

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement)


---

## 属性

 包含全局属性: true

- label

	The name of the menu as shown to the user. Used within nested menus, to provide a label through which the submenu can be accessed. Must only be specified when the parent element is a <a href="/zh-CN/docs/Web/HTML/Element/menu" aria-current="page"><code>&lt;menu&gt;</code></a> in the <em>context menu</em> state.
- type

	This attribute indicates the kind of menu being declared, and can be one of two values.
 <ul>
  <li><code>context</code> <abbr title="Deprecated. Not for use in new websites." class="icon icon-deprecated">
	<span class="visually-hidden">Deprecated</span>
</abbr> : Indicates the <em>popup menu</em> state, which represents a group of commands activated through another element. This might be as a button menu referenced by a <a href="/zh-CN/docs/Web/HTML/Element/button#attr-menu"><code>menu</code></a> attribute of a <a href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a> element, or as context menu for an element with a <a href="/en-US/docs/Web/HTML/Global_attributes#attr-contextmenu"><code>contextmenu</code></a> attribute. This value is the default if the attribute is missing and the parent element is also a <code>&lt;menu&gt;</code> element.</li>
  <li><code>toolbar</code>: Indicates the <em>toolbar</em> state, which represents a toolbar consisting of a series of commands for user interaction. This might be in the form of an unordered list of <a href="/zh-CN/docs/Web/HTML/Element/li"><code>&lt;li&gt;</code></a> elements, or, if the element has no <code>&lt;li&gt;</code> element children, flow content describing available commands. This value is the default if the attribute is missing.</li>
 </ul>
 "#####
}
