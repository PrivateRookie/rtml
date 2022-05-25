def! {
  link,
  Link,
  LinkArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<link>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/link)

---

**允许的内容**

无，这是一个<a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>。

**是否可忽略关闭标签**

鉴于这是一个空元素，开始标签必须存在，结束标签必须不存在。

**父标签**

任何可以接受元数据的元素.。如果使用了 <a href="/zh-CN/docs/Web/HTML/Global_attributes/itemprop">itemprop</a>属性，，则其父元素可以是任何可接受&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>&nbsp;的元素。

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLLinkElement)


---

## 属性

 包含全局属性: true

- "anonymous"

	此属性定义链接资源的字符编码。 该值为<a rel=" noopener" href="https://datatracker.ietf.org/doc/html/rfc2045" class="external">RFC 2045</a>中定义的字符集的空格和/或逗号分隔列表。 缺省值为 iso-8859-1。
 <div class="note notecard" id="sect10">
 <p><strong>使用说明：</strong>该属性已淘汰且禁止使用<strong>。要</strong>实现相同效果，可在外链资源中使用<code>Content-Type</code>&nbsp;HTTP header。</p>
 </div>
 
- "use-credentials"

	此属性的值显示了&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/link#attr-href" aria-current="page"><code>href</code></a>&nbsp;属性所定义的当前文档与链接文档的关系。 因此，该属性定义了与 rel 属性的值相比的反向关系。 该属性的<a href="/zh-CN/docs/Web/HTML/Link_types">Link type value</a>类似于<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/link#attr-rel"><code>rel</code></a>的可能值。
- &lt;audio&gt;

	此枚举属性指定在加载相关资源时是否必须使用 CORS. <a href="/en-US/docs/Web/HTML/CORS_enabled_image">启用 CORS 的图片</a>&nbsp;可以在 <a href="/zh-CN/docs/Web/HTML/Element/canvas"><code>&lt;canvas&gt;</code></a> 元素中重复使用，并避免其被<em>污染</em>. 可取的值如下：
 <dl>
  <dt id="anonymous"><code>"anonymous"</code></dt>
  <dd>会发起一个跨域请求 (即包含 <code>Origin:</code> HTTP 头). 但不会发送任何认证信息 (即不发送 cookie, X.509 证书和 HTTP 基本认证信息). 如果服务器没有给出源站凭证 (不设置&nbsp;<code>Access-Control-Allow-Origin:</code> HTTP 头), 资源就会被<em>污染并限制使用</em>.</dd>
  <dt id="use-credentials"><code>"use-credentials"</code></dt>
  <dd>会发起一个带有认证信息 (发送 cookie, X.509 证书和 HTTP 基本认证信息) 的跨域请求 (即包含 <code>Origin:</code> HTTP 头). 如果服务器没有给出源站凭证 (不设置&nbsp;<code>Access-Control-Allow-Origin:</code>&nbsp;HTTP 头), 资源就会被<em>污染并限制使用</em>.</dd>
 </dl>
 当不设置此属性时，资源将会不使用 CORS 加载 (即不发送 <code>Origin:</code> HTTP 头), 这将阻止其在 <a href="/zh-CN/docs/Web/HTML/Element/canvas"><code>&lt;canvas&gt;</code></a> 元素中进行使用。若设置了非法的值，则视为使用 <strong>anonymous</strong>. 前往&nbsp;<a href="/en-US/docs/Web/HTML/Attributes/crossorigin">CORS settings attributes</a> 获取更多信息。
- &lt;embed&gt;

	
 <p>仅对于<code>rel="stylesheet"</code>&nbsp;，<code>disabled</code>&nbsp;的 Boolean 属性指示是否应加载所描述的样式表并将其应用于文档。 如果在加载 HTML 时在 HTML 中指定了 Disabled，则在页面加载期间不会加载样式表。 相反，如果禁用属性更改为 false 或删除时，样式表将按需加载。</p>

 <p>但是，一旦加载样式表，对 Disabled 属性的值所做的更改将不再与<a href="/zh-CN/docs/Web/API/StyleSheet/disabled"><code>StyleSheet.disabled</code></a>&nbsp;属性的值有任何关系。 相反，更改此属性的值只是启用和禁用应用于文档的样式表表单。</p>

 <p>这与 StyleSheet 的 disable 属性不同； 将其更改为 true 会将样式表从文档的<a href="/zh-CN/docs/Web/API/Document/styleSheets"><code>document.styleSheets</code></a>&nbsp;列表中删除，并且在切换回 false 时不会自动重新加载样式表。</p>
 
- &lt;frame&gt;

	会发起一个带有认证信息 (发送 cookie, X.509 证书和 HTTP 基本认证信息) 的跨域请求 (即包含 <code>Origin:</code> HTTP 头). 如果服务器没有给出源站凭证 (不设置&nbsp;<code>Access-Control-Allow-Origin:</code>&nbsp;HTTP 头), 资源就会被<em>污染并限制使用</em>.
- &lt;iframe&gt;

	会发起一个跨域请求 (即包含 <code>Origin:</code> HTTP 头). 但不会发送任何认证信息 (即不发送 cookie, X.509 证书和 HTTP 基本认证信息). 如果服务器没有给出源站凭证 (不设置&nbsp;<code>Access-Control-Allow-Origin:</code> HTTP 头), 资源就会被<em>污染并限制使用</em>.
- &lt;image&gt;

	包含行内元数据，它是一个你用浏览器获取的资源文件的哈希值，以 base64 编码的方式加的密，这样用户能用它来验证一个获取到的资源，在传送时未被非法篡改，详情查看<a href="/en-US/docs/Web/Security/Subresource_Integrity">Subresource Integrity</a>。
- &lt;img&gt;

	此属性指明了被链接资源的语言。其意义仅供参考。可取的值参见&nbsp;<a href="https://www.ietf.org/rfc/bcp/bcp47.txt" class="external" rel=" noopener">BCP47</a>。仅当设置了 <a href="/zh-CN/docs/Web/HTML/Element/a#attr-href"><code>href</code></a> 属性时才应设置该属性。
- &lt;link rel=stylesheet&gt;

	属性在<code>&lt;link&gt;</code>元素上有特殊的语义。当用于<code>&lt;link rel="stylesheet"&gt;</code>时，它定义了一个<a href="/zh-CN/docs/Web/CSS/Alternative_style_sheets">首选样式表或备用样式表</a>。不正确地使用它可能会导致<a href="/zh-CN/docs/Correctly_Using_Titles_With_External_Stylesheets" class="page-not-created" title="This is a link to an unwritten page">样式表被忽略</a>。
- &lt;link&gt;

	此属性指定被链接资源的<a href="/zh-CN/docs/Glossary/URL">URL</a>。&nbsp;URL 可以是绝对的，也可以是相对的。
- &lt;object&gt;

	一个字符串，指示在获取资源时使用哪个引荐来源网址：
 <ul>
  <li><code>'no-referrer'</code>&nbsp;表示<a href="/zh-CN/docs/Web/HTTP/Headers/Referer"><code>Referer</code></a> 标头将不会发送。</li>
  <li><code>'no-referrer-when-downgrade'</code>&nbsp;的原始位置时不会发送任何<a href="/zh-CN/docs/Web/HTTP/Headers/Referer"><code>Referer</code></a>标头。 如果未指定其他政策，这是用户代理的默认行为。</li>
  <li><code>'origin'</code>&nbsp;意味着引荐来源网址将是页面的来源，大致是方案，主机和端口。</li>
  <li><code>'origin-when-cross-origin'</code>&nbsp;这意味着导航到其他来源将仅限于方案，主机和端口，而在同一来源上导航将包括引荐来源网址的路径。</li>
  <li><code>'unsafe-url'</code>&nbsp;意味着引荐来源网址将包含来源和路径（但不包括片段，密码或用户名）。 这种情况是不安全的，因为它可能会将来源和路径从受 TLS 保护的资源泄漏到不安全的来源。</li>
 </ul>
 
- &lt;picture&gt;

	指示资源的相对重要性。 优先级提示使用以下值委托：
- &lt;script&gt;

	此属性命名链接文档与当前文档的关系。 该属性必须是<a href="/zh-CN/docs/Web/HTML/Link_types">链接类型值</a>的用空格分隔的列表。
- &lt;track&gt;

	此属性的值提供有关可能在对象上执行的功能的信息。 这些值通常在使用时由 HTTP 协议给出，但是（出于与 “<strong>title</strong>” 属性类似的原因）将其预先包含在链接中可能是有用的。 例如，浏览器可能根据指定的方法选择不同的链接呈现方式。 可搜索的内容可能会得到其他图标，或者外部链接可能会显示离开当前站点的指示。 即使定义的浏览器 Internet Explorer 4 也无法很好地理解或支持此属性。
- &lt;video&gt;

	此属性标识下一个导航可能需要的资源，用户代理应检索该资源。 这允许用户代理在将来请求资源时更快地做出响应。
- *-image

	这个属性规定了外部资源适用的媒体类型。它的值必须是"<a href="/zh-CN/docs/Web/Guide/CSS/Media_queries">媒体查询</a>"。这个属性使得用户代理能选择最适合设备运行的媒体类型。
 <div id="sect5" class="note notecard"><strong>使用注意：</strong>
 <ul>
  <li>在 HTML 4 中，该属性只能是一组以空白符作为分隔的媒体描述文字，比如"<a href="/zh-CN/docs/Web/CSS/@media">媒体类型</a>"规定了该元素可取的属性，如 print、screen、aural、braille。HTML5 将该属性值扩展为任意类型的"<a href="/zh-CN/docs/CSS/Media_queries">媒体查询</a>"，"媒体查询"将 HTML4 的属性值都包括在内。</li>
  <li>不支持"<a href="/zh-CN/docs/CSS/Media_queries">CSS3 媒体查询</a>"的浏览器并不会强行识别这些链接，因此别忘了设置备用 link，即那些可用于 HTML4 的 link。</li>
 </ul>
 </div>
 
- @import

	这个属性被用于定义链接的内容的类型。这个属性的值应该是像 text/html，text/css 等 MIME 类型。这个属性常用的用法是定义链接的样式表，最常用的值是表明了 CSS 的 text/css。
- as

	该属性仅在<code>&lt;link&gt;</code>元素设置了 <code>rel="preload"</code> 或者 <code>rel="prefetch"</code> 时才能使用。它规定了<code>&lt;link&gt;元素</code>加载的内容的类型，对于内容的优先级、请求匹配、正确的<a href="/zh-CN/docs/Web/HTTP/CSP">内容安全策略</a>的选择以及正确的 <a href="/zh-CN/docs/Web/HTTP/Headers/Accept"><code>Accept</code></a>请求头的设置，这个属性是必需的。
- crossorigin

	定义具有已定义链接关系或将显示任何链接资源的呈现的框架或窗口名称。
- importScripts

	这个属性定义了包含相应资源的可视化媒体中的 icons 的大小。它只有在<a href="/zh-CN/docs/Web/HTML/Element/link#attr-rel" aria-current="page"><code>rel</code></a>包含 icon 的<a href="/zh-CN/docs/Web/HTML/Link_types">link 类型值</a>。它可能有如下的规则。
 <ul>
  <li><code>any</code>&nbsp;表示图标可以按矢量格式缩放到任意大小，例如<code>image/svg+xml。</code></li>
  <li>一个由空白符分隔的尺寸列表。每一个都以<code><em>&lt;width in pixels&gt;</em>x<em>&lt;height in pixels&gt;</em></code>&nbsp;或&nbsp;<code><em>&lt;width in pixels&gt;</em>X<em>&lt;height in pixels&gt;给出。</em></code>尺寸列表中的每一个尺寸都必须包含在资源里。</li>
 </ul>

 <div id="sect6" class="note notecard"><strong>用法注意：</strong>

 <ul>
  <li>大多数的 icon 格式只能存储一个 icon。因此绝大多数使用&nbsp;<a href="/zh-CN/docs/Web/HTML/Global_attributes#attr-sizes"><code>sizes</code></a>时只包含一个值。微软的 ICO 格式和苹果的 ICNS 格式都是这样，ICO 使用得更加广泛，推荐你使用它。</li>
  <li>苹果的 IOS 系统并不支持这个属性，于是苹果的 IPhone 以及 IPad 使用特殊的、非标准的&nbsp;<a href="/zh-CN/docs/Web/HTML/Link_types">link 类型值</a>去定义作为 Web Clip 或开始占位符：<code>apple-touch-icon</code>&nbsp;和&nbsp;<code>apple-touch-startup-icon。</code></li>
 </ul>
 </div>
 "#####
}
