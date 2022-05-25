def! {
  li,
  Li,
  LiArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<li>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/li)

---

**允许的内容**

<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">流式内容</a>

**是否可忽略关闭标签**

如果列表元素的后面紧随另一个 <a href="/zh-CN/docs/Web/HTML/Element/li" aria-current="page"><code>&lt;li&gt;</code></a> 元素，或者它的父元素中没有更多内容，结束标签可以省略。

**父标签**

<a href="/zh-CN/docs/Web/HTML/Element/ul"><code>&lt;ul&gt;</code></a>、 <a href="/zh-CN/docs/Web/HTML/Element/ol"><code>&lt;ol&gt;</code></a>、 或者 <a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a> 元素。过时的 <a href="/zh-CN/docs/Web/HTML/Element/dir"><code>&lt;dir&gt;</code></a> 也可以作为父元素，但是并不提倡。

[Dom API]()


---

## 属性

 包含全局属性: true

- type

	这个字符型属性表明了数字的类型：
 <ul>
  <li><code>a</code>: 小写字母</li>
  <li><code>A</code>: 大写字母</li>
  <li><code>i</code>: 小写罗马数字</li>
  <li><code>I</code>: 大写罗马数字</li>
  <li><code>1</code>: 数字</li>
 </ul>
 本属性值将覆盖&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/ol"><code>&lt;ol&gt;</code></a> 元素中的同名属性值（若存在）。

 <div class="note notecard" id="sect3"><strong>使用注解：</strong>&nbsp;本属性已废弃：使用&nbsp;CSS <a href="/zh-CN/docs/Web/CSS/list-style-type"><code>list-style-type</code></a> 属性来代替。</div>
 
- value

	这个整数型属性表明了本 <a href="/zh-CN/docs/Web/HTML/Element/li" aria-current="page"><code>&lt;li&gt;</code></a> 元素在有序列表（由 <a href="/zh-CN/docs/Web/HTML/Element/ol"><code>&lt;ol&gt;</code></a> 元素定义）中的序号。本属性值只能用数字，即使列表使用罗马数字或字母来展示。随后的列表条目会从设置的值开始计数。<strong>value</strong>&nbsp;属性对于无序列表 (<a href="/zh-CN/docs/Web/HTML/Element/ul"><code>&lt;ul&gt;</code></a>) 或者菜单 (<a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a>) 无效。
 <div class="note notecard" id="sect1"><strong>注：</strong>&nbsp;这个属性在 HTML 4 中废弃，但是在 HTML 5 中重新引入。</div>

 <div id="sect2" class="note notecard">
 <p><strong>注：</strong>&nbsp;在&nbsp;<span title="(Firefox 9.0 / Thunderbird 9.0 / SeaMonkey 2.6)">Gecko&nbsp;9.0</span> 之前，负值会错误地转换为 0。<span title="(Firefox 9.0 / Thunderbird 9.0 / SeaMonkey 2.6)">Gecko&nbsp;9.0</span> 开始，所有整数值都可以正确解析。</p>
 </div>
 "#####
}
