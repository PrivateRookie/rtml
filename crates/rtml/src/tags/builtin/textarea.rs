def_tag! {
  textarea,
  Textarea,
  TextareaArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<textarea>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/textarea)

---

**允许的内容**



**是否可忽略关闭标签**



**父标签**



[Dom API]()


---

## 属性

 包含全局属性: true

- autocapitalize

	iOS 的非标准属性（运行在 iOS 上的 Safari、Firefox、Chrome 都支持），文本是否自动首字母大写。在 iOS5 和之后的版本上有效。可能的值为：
 <ul>
  <li><code>none</code>: 禁用首字母大写。</li>
  <li><code>sentences</code>: 句子的首字母大写。</li>
  <li><code>words</code>: 单词或者字母的首字母大写。</li>
  <li><code>characters</code>: 全部字母大写。</li>
  <li><code>on</code>: <abbr class="icon icon-deprecated" title="Deprecated. Not for use in new websites.">
	<span class="visually-hidden">Deprecated</span>
</abbr>&nbsp;自 iOS 5 废弃。</li>
  <li><code>off</code>: <abbr title="Deprecated. Not for use in new websites." class="icon icon-deprecated">
	<span class="visually-hidden">Deprecated</span>
</abbr> 自 iOS 5 废弃。</li>
 </ul>
 
- autocomplete

	是否使用浏览器的记忆功能自动填充文本。可能的值有：
 <ul>
  <li><code>off</code>: 不使用浏览器的记忆自动填充，使用者必须输入他们想要输入的所有内容。或者网页提供了自己的自动填充方法。</li>
  <li><code>on</code>: 浏览器根据用户之前输入的内容或者习惯，在用户输入的时候给出相应输入提示。</li>
 </ul>

 <p>如果不指明<strong>autocomplete</strong>属性，浏览器会从父级的表单元素上解析是不是开启这个属性。表单元素可以是`textarea`元素的父级<a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a>或者`textarea`有跟表单相同的 id（参见下面的 form 属性）。更多请查看<a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a>的<a href="/zh-CN/docs/Web/HTML/Element/form#attr-autocomplete"><code>autocomplete</code></a> 属性。</p>
 
- autofocus

	页面加载完毕之后是否自动给本元素添加焦点。只有跟表格关联的才能使本属性生效。
- cols

	文本域的可视宽度。必须为正数，默认为 20&nbsp;(HTML5)。
- disabled

	禁用文本域。默认为 false。如果未指定，也可以从父级上如<a href="/zh-CN/docs/Web/HTML/Element/fieldset"><code>&lt;fieldset&gt;</code></a>继承而来。
- form

	指定跟自身相关联的表单。值必须为本文档内的表单的 ID，如果未指定，就是跟当前所在的表单元素相关联。这就允许你在文档的任意地方放置文本域元素。
- maxlength

	允许用户输入的最大字符长度 (Unicode) 。未指定表示无限长度。
- minlength

	允许用户输入的最小字符长度 (Unicode)&nbsp;
- name

	元素的名称。
- placeholder

	向用户提示可以在控件中输入的内容。 在渲染提示时，占位符文本中的回车符 (\r) 或换行符 (\n) 一定会被作为行断（换行）处理。
- readonly

	不允许用户修改元素内文本。和 <code>disabled</code> 属性不同的是，这个能让用户点击和选择元素内的文本。如果在表单里，这个元素的值还是会跟随表单一起提交。
- required

	提示用户这个元素的内容必填。
- rows

	元素的输入文本的行数（显示的高度）。
- spellcheck

	该属性设为 true 时，表明该元素会做拼写和语法检查。属性值为 default 时，表明元素有默认行为，可能会基于父元素的 spellcheck 值。属性值为 false 时，表明元素不做拼写和语法检查。
- wrap

	指定文本换行的方式。默认为 soft。可能的值为：
 <ul>
  <li>hard: 在文本到达元素最大宽度的时候，浏览器自动插入换行符 (CR+LF) 。比如指定 <code>cols</code>值。</li>
  <li>soft: 在到达元素最大宽度的时候，不会自动插入换行符。</li>
 </ul>
 "#####
}
