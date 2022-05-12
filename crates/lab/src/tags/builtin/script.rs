def! {
  script,
  Script,
  ScriptArg,
  doc:
  "zh-CN" = r#####"`<script>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/script)

---

**允许的内容**

动态脚本，如 <code>text/javascript</code>.

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

一些元素可以接受元数据内容，或者是一些元素可以接受短语元素。

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLScriptElement)


---

## 属性

 包含全局属性: true

- <span class="token tag"><span class="token tag"><span class="token punctuation">&lt;</span>script</span> <span class="token attr-name">src</span><span class="token attr-value"><span class="token punctuation attr-equals">=</span><span class="token punctuation">"</span><span class="token punctuation">"</span></span> <span class="token attr-name">crossorigin</span><span class="token attr-value"><span class="token punctuation attr-equals">=</span><span class="token punctuation">"</span>anonymous<span class="token punctuation">"</span></span><span class="token punctuation">&gt;</span></span><span class="token script"></span><span class="token tag"><span class="token tag"><span class="token punctuation">&lt;/</span>script</span><span class="token punctuation">&gt;</span></span>

	这个布尔属性被设定用来通知浏览器该脚本将在文档完成解析后，触发&nbsp;<code><a href="/en-US/docs/Web/API/Window/DOMContentLoaded_event" class="only-in-en-us" title="Currently only available in English (US)">DOMContentLoaded (en-US)</a></code>&nbsp;事件前执行。
- crossorigin

	那些没有通过标准<a href="/en-US/docs/Web/HTTP/CORS" title="Currently only available in English (US)" class="only-in-en-us">CORS (en-US)</a>检查的正常<code>script</code> 元素传递最少的信息到 <a href="/zh-CN/docs/Web/API/GlobalEventHandlers/onerror" title="window.onerror"><code>window.onerror</code></a>。可以使用本属性来使那些将静态资源放在另外一个域名的站点打印错误信息。参考 <a href="/zh-CN/docs/Web/HTML/CORS_settings_attributes">CORS 设置属性</a>了解对有效参数的更具描述性的解释。
- defer

	A cryptographic nonce (number used once) to whitelist inline scripts in a&nbsp;<a href="/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/script-src">script-src Content-Security-Policy</a>. The server must generate a unique nonce value each time it transmits a policy. It is critical to provide a nonce that cannot be guessed as bypassing a resource's policy is otherwise trivial.
- integrity

	Indicates which&nbsp;<a href="/en-US/docs/Web/API/Document/referrer">referrer</a>&nbsp;to send when fetching the script, or resources fetched by the script:
 <ul>
  <li><code>no-referrer</code>: The <a href="/zh-CN/docs/Web/HTTP/Headers/Referer"><code>Referer</code></a> header will not be sent.</li>
  <li><code>no-referrer-when-downgrade</code>&nbsp;(default): The <a href="/zh-CN/docs/Web/HTTP/Headers/Referer"><code>Referer</code></a> header will not be sent to <a href="/zh-CN/docs/Glossary/Origin">origin</a>s without <a href="/zh-CN/docs/Glossary/TLS">TLS</a> (<a href="/zh-CN/docs/Glossary/https">HTTPS</a>).</li>
  <li><code>origin</code>: The sent referrer will be limited to the origin of the referring page: its&nbsp;<a href="/en-US/docs/Archive/Mozilla/URIScheme">scheme</a>, <a href="/zh-CN/docs/Glossary/Host">host</a>, and <a href="/en-US/docs/Glossary/Port" class="only-in-en-us" title="Currently only available in English (US)">port <small>(en-US)</small></a>.</li>
  <li><code>origin-when-cross-origin</code>: The referrer sent to other origins will be limited to the scheme, the host, and the port. Navigations on the same origin will still include the path.</li>
  <li><code>same-origin</code>: A referrer will be sent for <a href="/zh-CN/docs/Glossary/Same-origin_policy">same origin</a>, but cross-origin requests will contain no referrer information.</li>
  <li><code>strict-origin</code>: Only send the origin of the document as the referrer when the protocol security level stays the same (e.g. HTTPS→HTTPS), but don't send it to a less secure destination (e.g. HTTPS→HTTP).</li>
  <li><code>strict-origin-when-cross-origin</code>: Send a full URL when performing a same-origin request, but only send the origin when the protocol security level stays the same (e.g.HTTPS→HTTPS), and send no header to a less secure destination (e.g. HTTPS→HTTP).</li>
  <li><code>unsafe-url</code>: The referrer will include the origin&nbsp;<em>and</em>&nbsp;the path (but not the&nbsp;<a href="/en-US/docs/Web/API/HTMLHyperlinkElementUtils/hash">fragment</a>,&nbsp;<a href="/en-US/docs/Web/API/HTMLHyperlinkElementUtils/password">password</a>, or&nbsp;<a href="/en-US/docs/Web/API/HTMLHyperlinkElementUtils/username">username</a>).&nbsp;<strong>This value is unsafe</strong>, because it leaks origins and paths from TLS-protected resources to insecure origins.</li>
 </ul>

 <p><strong>Note</strong>: An empty string value (<code>""</code>) is both the default value, and a fallback value if&nbsp;<code>referrerpolicy</code>&nbsp;is not supported. If&nbsp;<code>referrerpolicy</code>&nbsp;is not explicitly specified on the&nbsp;<code>&lt;script&gt;</code>&nbsp;element, it will adopt a higher-level referrer policy, i.e. one set on the whole document or domain. If a higher-level policy is not available,&nbsp;the empty string is treated as being equivalent to&nbsp;<code>no-referrer-when-downgrade</code>.</p>
 
- nomodule

	这个属性定义引用外部脚本的 URI，这可以用来代替直接在文档中嵌入脚本。指定了 src 属性的 script 元素标签内不应该再有嵌入的脚本。
- nonce

	该属性定义 script 元素包含或<code>src</code>引用的脚本语言。属性的值为<abbr title="多用途互联网邮件扩展类型">MIME</abbr>类型; 支持的<abbr title="多用途互联网邮件扩展类型">MIME</abbr>类型包括<code>text/javascript</code>, <code>text/ecmascript</code>, <code>application/javascript</code>, 和<code>application/ecmascript</code>。如果没有定义这个属性，脚本会被视作 JavaScript。
- referrerpolicy

	和 textContent 属性类似，本属性用于设置元素的文本内容。但和 textContent&nbsp; 不一样的是，本属性在节点插入到 DOM 之后，此属性被解析为可执行代码。 
- src

	如果存在，值必须和 “<code>utf-8</code>” 不区分大小写的匹配。当然声明&nbsp;<code>charset</code>&nbsp;是没有必要的，因为页面文档必须使用 UTF-8，而&nbsp;<code>script</code>&nbsp;元素会从页面文档中继承这个属性。
- type

	和 type 属性类似，这个属性定义脚本使用的语言。&nbsp;但是与 type 不同的是，这个属性的可能值从未被标准化过。请用<code>type</code>属性代替这个属性。"#####
}

