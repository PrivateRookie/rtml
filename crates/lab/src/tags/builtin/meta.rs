def! {
  meta,
  Meta,
  MetaArg,
  doc:
  "zh-CN" = r#####"`<meta>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/meta)

---

**允许的内容**

<dfn>允许的内容</dfn> 无，这是一个 <a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>

**是否可忽略关闭标签**

<dfn>标签省略</dfn>因为这是一个 void 元素，必须有开始标签而闭合标签可以省略

**父标签**

<dfn>允许的父元素</dfn><code>&lt;meta charset&gt;</code>, <code>&lt;meta http-equiv&gt;</code>: <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a> 元素。如果 <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/meta#attr-http-equiv"><code>http-equiv</code></a> 不是编码声明，它也可以放在<a href="/zh-CN/docs/Web/HTML/Element/noscript"><code>&lt;noscript&gt;</code></a>元素内，它本身在 <a href="/zh-CN/docs/Web/HTML/Element/head"><code>&lt;head&gt;</code></a>元素内部。

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement)


---

## 属性

 包含全局属性: true

- content

	此属性包含<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/meta#attr-http-equiv"><code>http-equiv</code></a> 或<a href="/zh-CN/docs/Web/HTML/Element/meta#attr-name" aria-current="page"><code>name</code></a> 属性的值，具体取决于所使用的值。
- http-equiv

	属性定义了一个编译指示指令。这个属性叫做 <code><strong>http-equiv</strong>(alent)</code> 是因为所有允许的值都是特定 HTTP 头部的名称，如下：
 <ul>
  <li><code>content-security-policy</code><br>
   它允许页面作者定义当前页的<a href="/en-US/docs/Web/HTTP/Headers/Content-Security-Policy">内容策略</a>。 内容策略主要指定允许的服务器源和脚本端点，这有助于防止跨站点脚本攻击。</li>
  <li><code>content-type</code><br>
   如果使用这个属性，其值必须是"<code>text/html; charset=utf-8</code>"。注意：该属性只能用于 <a href="/zh-CN/docs/Web/HTTP/Basics_of_HTTP/MIME_types">MIME type</a> 为 <code>text/html</code> 的文档，不能用于 MIME 类型为 XML 的文档。</li>
  <li><code>default-style</code>
   <p>设置默认 <a href="/zh-CN/docs/Web/CSS">CSS 样式表</a>组的名称。</p>
  </li>
  <li>
   <p><code>x-ua-compatible</code><br>
	如果指定，则 <code>content</code> 属性必须具有值 "<code>IE=edge</code>"。用户代理必须忽略此指示。</p>
  </li>
  <li><code>refresh</code><br>
   这个属性指定：
   <ul>
	<li>如果 <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/meta#attr-content"><code>content</code></a> 只包含一个正整数，则为重新载入页面的时间间隔 (秒)；</li>
	<li>如果 <a href="/zh-CN/docs/Web/HTML/Element/meta#attr-content" aria-current="page"><code>content</code></a> 包含一个正整数，并且后面跟着字符串 '<code>;url=</code>' 和一个合法的 URL，则是重定向到指定链接的时间间隔 (秒)</li>
   </ul>
  </li>
 </ul>

 <h5 id="可访问性相关考虑">可访问性相关考虑</h5>

 <p>设置了 <code>refresh</code> 值的页面可能有时间间隔太短的风险。使用诸如屏幕朗读这样的辅助技术来浏览网页的人可能会由于自动跳转而来不及读完或理解网页的内容。这样不经提示而突然进行的页面刷新也可能会让有视力障碍的人群感到迷惑。</p>

 <ul>
  <li><a class="only-in-en-us" href="/en-US/docs/Web/Accessibility/Understanding_WCAG/Operable" title="Currently only available in English (US)">MDN Understanding WCAG, Guideline 2.1 explanations (en-US)</a></li>
  <li><a title="Currently only available in English (US)" href="/en-US/docs/Web/Accessibility/Understanding_WCAG/Understandable" class="only-in-en-us">MDN Understanding WCAG, Guideline 3.1 explanations (en-US)</a></li>
  <li><a class="external" href="https://www.w3.org/TR/UNDERSTANDING-WCAG20/time-limits-required-behaviors.html" rel=" noopener">Understanding Success Criterion 2.2.1 | W3C Understanding WCAG 2.0</a></li>
  <li><a href="https://www.w3.org/TR/UNDERSTANDING-WCAG20/time-limits-postponed.html" class="external" rel=" noopener">Understanding Success Criterion 2.2.4 | W3C Understanding WCAG 2.0</a></li>
  <li><a rel=" noopener" href="https://www.w3.org/TR/UNDERSTANDING-WCAG20/consistent-behavior-no-extreme-changes-context.html" class="external">Understanding Success Criterion 3.2.5 | W3C Understanding WCAG 2.0</a></li>
 </ul>
 
- name

	<code>name</code> 和 <code>content</code> 属性可以一起使用，以名 - 值对的方式给文档提供元数据，其中 name 作为元数据的名称，content 作为元数据的值。"#####
}

