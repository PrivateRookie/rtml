def_tag! {
  form,
  Form,
  FormArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<form>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/form)

---

**允许的内容**

<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>, but not containing <code>&lt;form&gt;</code> elements

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

可以是 HTML 的<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">任何标签</a>

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLFormElement)


---

## 属性

 包含全局属性: true

- accept

	一个逗号分隔的列表，包括服务器能接受的内容类型。
 <div class="note notecard" id="sect4"><strong>此属性已在 HTML5 中被移除并且不再被使用。</strong>作为替代，可以使用&nbsp;<code>&lt;input type=file&gt;</code>&nbsp;元素中的 <a href="/zh-CN/docs/Web/HTML/Element/Input#attr-accept"><code>accept</code></a> 属性。</div>
 
- accept-charset

	一个空格分隔或逗号分隔的列表，此列表包括了服务器支持的字符编码。浏览器以这些编码被列举的顺序使用它们。默认值是一个保留字符串 <code>"UNKNOWN"</code>。此字符串指的是，和包含此表单元素的文档相同的编码。<br>
 在之前版本的 HTML 中，不同的字符编码可以用空格或逗号分隔。在 HTML5 中，只有空格可以允许作为分隔符。
- action

	处理表单提交的 URL。这个值可被&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a>、<code><a href="/zh-CN/docs/Web/HTML/Element/Input/submit">&lt;input type="submit"&gt;</a></code>&nbsp;或&nbsp;<code><a href="/zh-CN/docs/Web/HTML/Element/Input/image">&lt;input type="image"&gt;</a></code>&nbsp;元素上的 <a href="/zh-CN/docs/Web/HTML/Element/button#attr-formaction"><code>formaction</code></a> 属性覆盖。
- autocapitalize

	这是一个被 iOS Safari 使用的非标准属性。当用户在一些表单的文本后代控件中，输入/编辑一些文本值时，此属性控制了这些文本值的首字母是否大写或者使用其他的大写样式。如果 <code>autocapitalize</code> 属性在某个单独的表单后代控件被指定的话，那么此单独的设定会覆盖原来表单范围内的 <code>autocapitalize</code> 设定。默认值为 <code>sentences</code>。可以选择的值如下：
 <ul>
  <li><code>none</code>：完全禁用自动首字母大写。</li>
  <li><code>sentences</code>：自动对每句话首字母大写。</li>
  <li><code>words</code>：自动对每个单词首字母大写。</li>
  <li><code>characters</code>：自动大写所有的字母。</li>
 </ul>
 
- autocomplete

	用于指示 input 元素是否能够拥有一个默认值，此默认值是由浏览器自动补全的。此设定可以被属于此表单的子元素的 <code>autocomplete</code> 属性覆盖。 可能的值有：
 <ul>
  <li><code>off</code>：浏览器可能不会自动补全条目（在疑似登录表单中，浏览器倾向于忽略该值，而提供密码自动填充功能，参见 <a href="/zh-CN/docs/Web/Security/Securing_your_site/Turning_off_form_autocompletion#禁用自动填充">自动填充属性和登录</a>）</li>
  <li><code>on</code>：浏览器可自动补全条目</li>
 </ul>


 
- enctype

	当 <code>method</code> 属性值为 <code>post</code> 时，<code>enctype</code> 就是将表单的内容提交给服务器的 <a href="http://en.wikipedia.org/wiki/Mime_type" class="external" rel=" noopener">MIME 类型</a> 。可能的取值有：
 <ul>
  <li><code>application/x-www-form-urlencoded</code>：未指定属性时的默认值。</li>
  <li><code>multipart/form-data</code>：当表单包含&nbsp;<code>type=file</code>&nbsp;的 <a href="/zh-CN/docs/Web/HTML/Element/Input"><code>&lt;input&gt;</code></a> 元素时使用此值。</li>
  <li><code>text/plain</code>：出现于 HTML5，用于调试。</li>
 </ul>

 <p>这个值可被&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a>、<code><a href="/zh-CN/docs/Web/HTML/Element/Input/submit">&lt;input type="submit"&gt;</a></code>&nbsp;或&nbsp;<code><a href="/zh-CN/docs/Web/HTML/Element/Input/image">&lt;input type="image"&gt;</a></code>&nbsp;元素上的 <a href="/zh-CN/docs/Web/HTML/Element/button#attr-formenctype"><code>formenctype</code></a> 属性覆盖。</p>
 
- method

	浏览器使用这种 <a href="/en-US/docs/Web/HTTP">HTTP</a> 方式来提交 表单。可能的值有：
 <ul>
  <li><code>post</code>：指的是 HTTP <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html#sec9.5" class="external" rel=" noopener">POST 方法</a>；表单数据会包含在表单体内然后发送给服务器。</li>
  <li><code>get</code>：指的是 HTTP <a rel=" noopener" href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html#sec9.3" class="external">GET 方法</a>；表单数据会附加在 <code>action</code> 属性的 URL 中，并以 '?' 作为分隔符，<a href="/zh-CN/docs/Glossary/Idempotent">没有副作用</a>&nbsp;时使用这个方法。</li>
  <li><code>dialog</code>：如果表单在 <a href="/zh-CN/docs/Web/HTML/Element/dialog"><code>&lt;dialog&gt;</code></a> 元素中，提交时关闭对话框。</li>
 </ul>

 <p>此值可以被 <a href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a>、<code><a href="/zh-CN/docs/Web/HTML/Element/Input/submit">&lt;input type="submit"&gt;</a></code> 或 <code><a href="/zh-CN/docs/Web/HTML/Element/Input/image">&lt;input type="image"&gt;</a></code> 元素中的 <a href="/zh-CN/docs/Web/HTML/Element/button#attr-formmethod"><code>formmethod</code></a> 属性覆盖。</p>
 
- name

	表单的名称。HTML 4 中不推荐（应使用 <code>id</code>）。在 HTML 5 中，该值必须是所有表单中独一无二的，而且不能是空字符串。
- novalidate

	此布尔值属性表示提交表单时不需要验证表单。 如果没有声明该属性（因此表单需要通过验证）。该属性可以被表单中的 <a href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a>、<code><a href="/zh-CN/docs/Web/HTML/Element/Input/submit">&lt;input type="submit"&gt;</a></code> 或 <code><a href="/zh-CN/docs/Web/HTML/Element/Input/image">&lt;input type="image"&gt;</a></code> 元素中的 <a href="/zh-CN/docs/Web/HTML/Element/button#attr-formnovalidate"><code>formnovalidate</code></a> 属性覆盖。
- rel

	根据 value 创建一个超链接或 Creates a hyperlink or annotation depending on the value, see the&nbsp;<a href="/en-US/docs/Web/HTML/Attributes/rel" class="only-in-en-us" title="Currently only available in English (US)"> (en-US)</a><a href="#attr-rel" id="attr-rel"><b><code>rel</code></b></a>&nbsp;attribute for details.
- target

	表示在提交表单之后，在哪里显示响应信息。在 HTML 4 中，这是一个&nbsp;frame 的名字/关键字对。在 HTML5 里，这是一个<em>浏览上下文</em> 的名字/关键字（如标签页、窗口或&nbsp;iframe）。下述关键字有特别含义：
 <ul>
  <li><code>_self</code>：默认值。在相同浏览上下文中加载。</li>
  <li><code>_blank</code>：在新的未命名的浏览上下文中加载。</li>
  <li><code>_parent</code>：在当前上下文的父级浏览上下文中加载，如果没有父级，则与 _self 表现一致。</li>
  <li><code>_top</code>：在最顶级的浏览上下文中（即当前上下文的一个没有父级的祖先浏览上下文），如果没有父级，则与 _self 表现一致。</li>
 </ul>

 <p>此值可以被 <a href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a>、 <code><a href="/zh-CN/docs/Web/HTML/Element/Input/submit">&lt;input type="submit"&gt;</a></code> 或 <code><a href="/zh-CN/docs/Web/HTML/Element/Input/image">&lt;input type="image"&gt;</a></code> 元素中的 <a href="/zh-CN/docs/Web/HTML/Element/button#attr-formtarget"><code>formtarget</code></a> 属性覆盖。</p>
 "#####
}
