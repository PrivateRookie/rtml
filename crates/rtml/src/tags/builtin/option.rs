def_tag! {
  option,
  Option_,
  OptionArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<option>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/option)

---

**允许的内容**

<dfn>允许的内容</dfn>带有最终转义字符（例如 <code>&amp;eacute;</code>）的文本

**是否可忽略关闭标签**

<dfn>标记省略</dfn> 开始标记是必需的。如果此元素紧接着是另一个 <code>&lt;option&gt;</code> 元素或<a href="/zh-CN/docs/Web/HTML/Element/optgroup"><code>&lt;optgroup&gt;</code></a>, 或者父元素没有其他内容，则结束标记是可选的。

**父标签**

<dfn>Implicit ARIA role</dfn><code><a rel=" noopener" href="https://w3c.github.io/aria/#option" class="external">option</a></code>

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLOptionElement)


---

## 属性

 包含全局属性: true

- disabled

	如果设置了这个布尔属性，则该选项不可选。浏览器通常会将这种控件显示为灰色，并且不再接受任何浏览器事件，例如鼠标点击或者焦点相关的事件。如果这个属性没有设置，而这个元素的其中一个父元素是被禁用的 <a href="/zh-CN/docs/Web/HTML/Element/optgroup"><code>&lt;optgroup&gt;</code></a> 元素，则这个元素仍然是禁用的&nbsp;。
- label

	这个属性是用于表示选项含义的文本。如果&nbsp;<code><strong>label</strong></code>&nbsp;属性没有定义，它的值就是元素文本内容。
- selected

	这个布尔属性存在时表明这个选项是否一开始就被选中。如果&nbsp;<code>&lt;option&gt;</code>&nbsp;元素是 <a href="/zh-CN/docs/Web/HTML/Element/select"><code>&lt;select&gt;</code></a> 元素的子元素，并且 <a href="/zh-CN/docs/Web/HTML/Element/select"><code>&lt;select&gt;</code></a> 元素的&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/select#attr-multiple"><code>multiple</code></a> 属性没有设置，则 <a href="/zh-CN/docs/Web/HTML/Element/select"><code>&lt;select&gt;</code></a> 元素中只有一个 <code>&lt;option&gt;</code>&nbsp;元素可以拥有 <code>selected</code>&nbsp;属性。
- value

	这个属性的值表示该选项被选中时提交给表单的值。如果省略了这个属性，值就从选项元素的文本内容中获取。"#####
}
