def! {
  menuitem,
  Menuitem,
  MenuitemArg,
  doc:
  "zh-CN" = r#####"`<menuitem>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/menuitem)

---

**允许的内容**

None，这是一个<a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>。

**是否可忽略关闭标签**

必须有开始标签和结束标签。

**父标签**

The <a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a> element, where that element is in the <em>popup menu</em> state. (If specified, the <code>type</code> attribute of the <a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a> element must be <code>popup</code>; if missing, the parent element of the <a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a> must itself be a <a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a> in the <em>popup menu</em> state.)

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement)


---

## 属性

 包含全局属性: true

- checked

	布尔值，指示是否选择了命令。只能作为属性使用在 <code>checkbox</code> 或 <code>radio</code> 中。
- command

	指定一个单独元素的 ID，指示要间接调用的命令。在包含属性的菜单项中也不能使用。<code>checked</code>、<code>disabled</code>、<code>icon</code>、<code>label</code>、<code>radiogroup</code> 或 <code>type</code>。
- default

	布尔值，表示使用与菜单主题元素相同的命令。（如 <code>button</code> 或 <code>input</code>）。
- disabled

	布尔值，表示命令在当前状态下不可用。请注意，禁用与隐藏不同；禁用的属性适用于任何环境变化可能导致命令相关的上下文中。
- icon

	图片 URL，用于提供图片来表示命令。
- label

	展示给用户一个命令的名字，当 <code>command</code> 属性不存在时是必须的。
- radiogroup

	此属性指定要切换为单选按钮时，选定的一组命令的名称。只能作为 radio 的属性使用。
- type

	这个属性指定命令的类型，可以为以下三个命令之一。
 <ul>
  <li><code>command</code>：有关联动作的常规命令。这是缺少时的值默认值。</li>
  <li><code>checkbox</code>：代表一个命令可以在两个不同状态之间的切换。</li>
  <li><code>radio</code>：代表一组单选按钮，可切换为命令中的一个选择。</li>
 </ul>
 "#####
}

