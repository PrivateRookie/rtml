def! {
  a,
  A,
  AArg,
  doc:
  "zh-CN" = r#####"`<a>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/a)

---

**允许的内容**

可见的内容（Transparent），包含流内容（不包括交互式内容）或文字内容（phrasing content）。

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

接受短语内容的任何元素或接受流内容的任何元素，但始终不接受 &lt;a&gt; 元素（根据对称的逻辑原理，如果 &lt;a&gt; 标记作为父元素，不能具有交互内容，则相同的 &lt;a&gt; 内容不能具有 &lt;a&gt; 标记作为其父元素）。

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLAnchorElement)


---

## 属性

 包含全局属性: true

- charset

	此属性定义链接资源的字符编码。该值是一个空格- 和/或逗号分隔的<a style="color: rgb(102, 153, 204); text-decoration: none; padding-right: 16px; background-color: rgb(255, 255, 255); font-family: 'Lucida Grande', 'Lucida Sans Unicode', 'DejaVu Sans', Lucida, Arial, Helvetica, sans-serif; font-size: 14px; font-style: normal; font-variant: normal; font-weight: normal; letter-spacing: normal; line-height: 22px; text-align: start; text-indent: 0px; text-transform: none; white-space: normal;" href="https://tools.ietf.org/html/rfc2045" class="external" rel=" noopener">RFC 2045</a>中定义的字符集列表。默认值是 ISO-8859-1。
 <div id="sect8" class="note notecard">
 <p><strong>使用说明：</strong>在 HTML5 中该属性已作废，不应使用。 为了实现其效果，使用 HTTP content - type header 的链接资源。</p>
 </div>
 
- coords

	对于使用对象的形状，此属性使用逗号分隔的数字列表来定义对象在页面上的坐标。
- download

	此属性指示浏览器下载 URL 而不是导航到它，因此将提示用户将其保存为本地文件。如果属性有一个值，那么此值将在下载保存过程中作为预填充的文件名（如果用户需要，仍然可以更改文件名）。此属性对允许的值没有限制，但是 <code>/</code> 和 <code>\</code> 会被转换为下划线。大多数文件系统限制了文件名中的标点符号，故此，浏览器将相应地调整建议的文件名。
 <div class="note notecard" id="sect3"><strong>注意：</strong>
 <ul>
  <li>此属性仅适用于<a href="/zh-CN/docs/Web/Security/Same-origin_policy">同源 URL</a>。</li>
  <li>尽管 HTTP URL 需要位于同一源中，但是可以使用 <a href="/zh-CN/docs/Web/API/URL/createObjectURL"><code>blob:</code> URL</a> 和 <a href="/zh-CN/docs/Web/HTTP/Basics_of_HTTP/Data_URIs"><code>data:</code> URL</a> ，以方便用户下载使用 JavaScript 生成的内容（例如使用在线绘图 Web 应用程序创建的照片）。</li>
  <li>如果 HTTP 头中的 <a href="/zh-CN/docs/Web/HTTP/Headers/Content-Disposition">Content-Disposition</a> 属性赋予了一个不同于此属性的文件名，HTTP 头属性优先于此属性。</li>
  <li>如果 HTTP 头属性 <code>Content-Disposition</code> 被设置为 inline（即 <code>Content-Disposition='inline'</code>），那么 Firefox 优先考虑 HTTP 头 <code>Content-Disposition</code>download 属性。</li>
 </ul>
 </div>
 
- href

	包含超链接指向的 URL 或 URL 片段。
- hreflang

	该属性用于指定链接文档的人类语言。其仅提供建议，并没有内置的功能。hreflang 允许的值取决于 HTML5 <a class="external" href="https://www.ietf.org/rfc/bcp/bcp47.txt" rel=" noopener">BCP47</a>。
- name

	该属性在页面中定义锚点的目标位置时是必须的。 <strong>name</strong> 的值类似于 ID 核心属性值，该属性值在文档中是唯一的且由数字字母标示符所组成的。在 HTML 4.01 规范中，<strong>id</strong> 和 <strong>name</strong> 都可以使用 &lt;a&gt; 元素，只要他们有相同的值。
 <div class="note notecard" id="sect9">
 <p><strong>使用说明：</strong> 该属性在 HTML5 中是过时的，使用 <a href="/en-US/docs/Web/HTML/Global_attributes#attr-id" class="only-in-en-us" title="Currently only available in English (US)">全局属性 id (en-US)</a> 来代替。</p>
 </div>
 
- ping

	包含一个以空格分隔的 url 列表，当跟随超链接时，将由浏览器 (在后台) 发送带有正文 PING 的 POST 请求。通常用于跟踪。
- referrerpolicy

	表明在获取 URL 时发送哪个提交者（referrer）:
 <ul>
  <li><code>"no-referrer"</code> 表示 <code>Referer:</code> 头将不会被发送。</li>
  <li><code>"no-referrer-when-downgrade"</code> 表示当从使用 HTTPS 的页面导航到不使用 TLS(HTTPS) 的来源 时不会发送 <code>Referer:</code> 头。如果没有指定策略，这将是用户代理的默认行为。</li>
  <li><code>"origin"</code> 表示 referrer 将会是页面的来源，大致为这样的组合：主机和端口（不包含具体的路径和参数的信息）。</li>
  <li>"origin-when-cross-origin" 表示导航到其它源将会限制为这种组合：主机 + 端口，而导航到相同的源将会只包含 referrer 的路径。</li>
  <li><code>'strict-origin-when-cross-origin'</code></li>
  <li><code>"unsafe-url"</code> 表示 referrer 将会包含源和路径（domain + path）（但是不包含密码或用户名的片段）。这种情况是不安全的，因为它可能会将安全的 URLs 数据泄露给不安全的源。</li>
 </ul>
 
- rel

	该属性指定了目标对象到链接对象的关系。该值是空格分隔的<a href="/zh-CN/docs/Web/HTML/Link_types">列表类型值</a>。
- rev

	该属性用于指定当前文档与被链接文档的关系。用于 &lt;a&gt; 标签的可选属性 rel 和 rev 分别表示源文档与目标文档之间正式的关系和方向。rel 属性指定从源文档到目标文档的关系，而 rev 属性则指定从目标文档到源文档的关系。这两种属性可以在 &lt;a&gt; 标签中同时使用。对于网页编者来说，这个属性很有用，可以通过它来查看外链的来源。
- shape

	此属性用于定义一个可选的超链接相关的一个数字来创建图像映射区域，该属性的值是圆，默认情况下，多边形，矩形。 所述的<strong>coords</strong> 属性的格式取决于形状的值。对于圆，该值的 x，y，r 其中 x 和 y 为圆心像素坐标，r 是像素值半径。对于矩形，该<strong>coords</strong> 属性应该是 x，y，w，h。x，y 值定义的矩形的左上角的位置，而 w 和 h 分别定义宽度和高度。多边形形状的值需要 x1，y1，x2，y2，......值来定义 coords。每对 x，y 定义多边形的一个点，连续点由直线接合和，最后一个点连接到第一个点。默认为形状的值需要将整个封闭区域，典型的图像，可以使用。
 <div class="note notecard" id="sect10"><strong>说明：</strong>建议使用 <a href="/zh-CN/docs/Web/HTML/Element/img#attr-usemap">使用 map 属性</a>在 <a href="/zh-CN/docs/Web/HTML/Element/img"><code>&lt;img&gt;</code></a> 元素和相关的<a href="/zh-CN/docs/Web/HTML/Element/map"><code>&lt;map&gt;</code></a>元素中来定义热点而不是用形状属性。</div>
 
- target

	该属性指定在何处显示链接的资源。 取值为标签（tab），窗口（window），或框架（iframe）等浏览上下文的名称或其他关键词。以下关键字具有特殊的意义：
 <ul>
  <li><code>_self</code>: 当前页面加载，即当前的响应到同一 HTML 4 frame（或 HTML5 浏览上下文）。此值是默认的，如果没有指定属性的话。</li>
  <li><code>_blank</code>: 新窗口打开，即到一个新的未命名的 HTML4 窗口或 HTML5 浏览器上下文</li>
  <li><code>_parent</code>: 加载响应到当前框架的 HTML4 父框架或当前的 HTML5 浏览上下文的父浏览上下文。如果没有 parent 框架或者浏览上下文，此选项的行为方式与 <code>_self</code> 相同。</li>
  <li><code>_top</code>: IHTML4 中：加载的响应成完整的，原来的窗口，取消所有其它 frame。 HTML5 中：加载响应进入顶层浏览上下文（即，浏览上下文，它是当前的一个的祖先，并且没有 parent）。如果没有 parent 框架或者浏览上下文，此选项的行为方式相同_self</li>
 </ul>
 
- type

	该属性指定在一个 <a href="/zh-CN/docs/Glossary/MIME_type">MIME type</a> 链接目标的形式的媒体类型。其仅提供建议，并没有内置的功能。"#####
}

