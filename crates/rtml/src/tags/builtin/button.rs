def_tag! {
  button,
  Button,
  ButtonArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<button>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/button)

---

**允许的内容**

<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>.

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。&nbsp;

**父标签**

任意可容纳&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>&nbsp;的元素。

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLButtonElement)


---

## 属性

 包含全局属性: true

- autocomplete

	该属性在 <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a>上的使用并未标准化，只有 Firefox 允许。不像其它浏览器，Firefox 默认在页面加载时持续禁用 Button 的动态状态（<a rel=" noopener" href="https://stackoverflow.com/questions/5985839/bug-with-firefox-disabled-attribute-of-input-not-resetting-when-refreshing" class="external">Firefox persists the dynamic disabled state</a>&nbsp;）。将此属性设置为<code style="font-style: normal;">off</code>&nbsp;(i.e.&nbsp;<code style="font-style: normal;">autocomplete="off"</code>) 关闭此特性。参见&nbsp;<a href="https://bugzilla.mozilla.org/show_bug.cgi?id=654072" rel=" noopener" class="external">bug&nbsp;654072</a>。
- autofocus

	一个布尔属性，用于指定当页面加载时按钮必须有输入焦点，除非用户重写，例如通过不同控件键入。只有一个表单关联元素可以指定该属性。
- disabled

	
 <p>此布尔属性表示用户不能与 button 交互。如果属性值未指定，button 继承包含元素，例如<a href="/zh-CN/docs/Web/HTML/Element/fieldset"><code>&lt;fieldset&gt;</code></a>；如果没有设置<strong>disabled</strong>属性的包含元素，button 将可交互。</p>

 <p>不像其它浏览器，Firefox 默认在页面加载时持续禁用 Button 的动态状态。使用<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/button#attr-autocomplete"><code>autocomplete</code></a>属性可控制此特性。</p>
 
- form

	表示 button 元素关联的 form 元素（它的表单拥有者）。此属性值必须为同一文档中的一个<a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a>元素的<strong>id</strong>属性。如果此属性未指定，&lt;button&gt;元素必须是 form 元素的后代。利用此属性，你可以将&lt;button&gt;元素放置在文档内的任何位置，而不仅仅是作为他们 form 元素的后代。
- formaction

	表示程序处理 button 提交信息的 URI。如果指定了，将重写 button 表单拥有者的<a href="/zh-CN/docs/Web/HTML/Element/form#attr-action"><code>action</code></a>属性。
- formenctype

	如果 button 是 submit 类型，此属性值指定提交表单到服务器的内容类型。可选值：
 <ul>
  <li><code>application/x-www-form-urlencoded</code>: 未指定时的默认值。</li>
  <li><code>multipart/form-data</code>: 如果使用<a href="/zh-CN/docs/Web/HTML/Element/Input#attr-type"><code>type</code></a>属性的<a href="/zh-CN/docs/Web/HTML/Element/Input"><code>&lt;input&gt;</code></a>元素设置文件，使用此值。</li>
  <li><code>text/plain</code></li>
 </ul>

 <p>如果指定此属性，它将重写 button 的表单拥有者的<a href="/zh-CN/docs/Web/HTML/Element/form#attr-enctype"><code>enctype</code></a>属性。</p>
 
- formmethod

	如果 button 是 submit 类型，此属性指定浏览器提交表单使用的 HTTP 方法。可选值：
 <ul>
  <li><code>post</code>: 来自表单的数据被包含在表单内容中，被发送到服务器。</li>
  <li><code>get</code>: &nbsp;来自表单的数据以'?'作为分隔符被附加到 form 的<strong>URI</strong>属性中，得到的 URI 被发送到服务器。当表单没有副作用，且仅包含 ASCII 字符时使用这种方法。</li>
 </ul>

 <p>如果指定了，此属性会重写 button 拥有者的<a href="/zh-CN/docs/Web/HTML/Element/form#attr-method"><code>method</code></a>属性。</p>
 
- formnovalidate

	如果 button 是 submit 类型，此布尔属性指定当表单被提交时不需要验证。如果指定了，它会重写 button 拥有者的<a href="/zh-CN/docs/Web/HTML/Element/form#attr-novalidate"><code>novalidate</code></a>属性。
- formtarget

	如果 button 是 submit 类型，此属性指定一个名称或关键字，表示接收提交的表单后在哪里显示响应。这是一个浏览上下文（例如 tab，window 或内联框架）的名称或关键字。如果指定了，它会重写 button 拥有者的<a href="/zh-CN/docs/Web/HTML/Element/form#attr-target"><code>target</code></a> 属性。关键字如下：
 <ul>
  <li><code>_self</code>: 在同一个浏览上下文中加载响应作为当前的。未指定时此值为默认值。</li>
  <li><code>_blank</code>: &nbsp;在一个新的不知名浏览上下文中加载响应。</li>
  <li><code>_parent</code>: 在当前浏览上下文父级中加载响应。如果没有父级的，此选项将按_self 执行。</li>
  <li><code>_top</code>: &nbsp;在顶级浏览上下文（即当前浏览上下文的祖先，且没有父级）中架加载响应。如果没有顶级的，此选项将按_self 执行。</li>
 </ul>
 
- name

	button 的名称，与表单数据一起提交。
- type

	button 的类型。可选值：
 <ul>
  <li><code>submit</code>:&nbsp; 此按钮将表单数据提交给服务器。如果未指定属性，或者属性动态更改为空值或无效值，则此值为默认值。</li>
  <li><code>reset</code>: &nbsp;此按钮重置所有组件为初始值。</li>
  <li><code>button</code>: 此按钮没有默认行为。它可以有与元素事件相关的客户端脚本，当事件出现时可触发。</li>
  <li>menu: 此按钮打开一个由指定<a href="/zh-CN/docs/Web/HTML/Element/menu"><code>&lt;menu&gt;</code></a>元素进行定义的弹出菜单。</li>
 </ul>
 
- value

	button 的初始值。它定义的值与表单数据的提交按钮相关联。当表单中的数据被提交时，这个值便以参数的形式被递送至服务器。"#####
}
