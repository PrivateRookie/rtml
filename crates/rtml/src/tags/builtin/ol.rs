def! {
  ol,
  Ol,
  OlArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<ol>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/ol)

---

**允许的内容**

Zero or more <a href="/zh-CN/docs/Web/HTML/Element/li"><code>&lt;li&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/script"><code>&lt;script&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/template"><code>&lt;template&gt;</code></a> elements.

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">flow content</a>.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement)


---

## 属性

 包含全局属性: true

- reversed

	此布尔值属性指定列表中的条目是否是倒序排列的，即编号是否应从高到低反向标注。
- start

	一个整数值属性，指定了列表编号的起始值。此属性的值应为阿拉伯数字，尽管列表条目的编号类型 <code>type</code> 属性可能指定为了罗马数字编号等其他类型的编号。比如说，想要让元素的编号从英文字母 "d" 或者罗马数字 "iv" 开始，都应当使用 <code>start="4"</code>。
 <div id="sect2" class="note notecard"><strong>Note</strong>: 这个属性在 HTML4 中弃用，但是在 HTML5 中被重新引入。</div>
 
- type

	设置编号的类型：
 <ul>
  <li><code>a</code> 表示小写英文字母编号</li>
  <li><code>A</code> 表示大写英文字母编号</li>
  <li><code>i</code> 表示小写罗马数字编号</li>
  <li><code>I</code> 表示大写罗马数字编号</li>
  <li><code>1</code> 表示数字编号（默认）</li>
 </ul>

 <p>编号类型适用于整个列表，除非在 <code>&lt;ol&gt;</code> 元素的 <a href="/zh-CN/docs/Web/HTML/Element/li"><code>&lt;li&gt;</code></a> 元素中使用不同的 <a href="/zh-CN/docs/Web/HTML/Element/li#attr-type"><code>type</code></a> 属性。</p>

 <div id="sect3" class="note notecard"><strong>Note: </strong>这个属性在 HTML4 中弃用，但是在 HTML5 中被重新引入。除非列表中序号很重要（比如，在法律或者技术文件中条目通常被需要所引用），否则请使用 CSS <a href="/zh-CN/docs/Web/CSS/list-style-type"><code>list-style-type</code></a> 属性替代。</div>
 "#####
}

