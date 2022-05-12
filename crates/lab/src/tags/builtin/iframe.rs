def! {
  iframe,
  Iframe,
  IframeArg,
  doc:
  "zh-CN" = r#####"`<iframe>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/iframe)

---

**允许的内容**



**是否可忽略关闭标签**



**父标签**



[Dom API]()


---

## 属性

 包含全局属性: true

- align

	此元素相对于周围元素的对齐方式。
- allow

	用于为<code>&lt;iframe&gt;</code>指定其<a href="/zh-CN/docs/Web/HTTP/Feature_Policy">特征策略</a>.
- allowfullscreen

	设置为<code>true</code>时，可以通过调用 <code>&lt;iframe&gt;</code> 的 <a title="requestFullscreen()" href="/zh-CN/docs/Web/API/Element/requestFullScreen"><code>requestFullscreen()</code></a> 方法激活全屏模式。
- allowpaymentrequest

	设置为<code>true</code>时，跨域的 <code>&lt;iframe&gt;</code> 就可以调用 <a href="/en-US/docs/Web/API/Payment_Request_API">Payment Request API</a>。
- auto

	不指定优先级。浏览器根据自身情况决定资源的加载顺序
- csp

	对嵌入的资源配置<a href="/zh-CN/docs/Web/HTTP/CSP">内容安全策略</a>。 查看 <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/API/HTMLIFrameElement/csp"><code>HTMLIFrameElement.csp</code> <small>(en-US)</small></a> 获取详情。
- frameborder

	值为<code>1</code>（默认值）时，显示此框架的边框。值为<code>0</code>时移除边框。此属性已不赞成使用，请使用 CSS 属性 <a href="/zh-CN/docs/Web/CSS/border"><code>border</code></a> 代替。
- height

	以 CSS 像素格式<span class="badge inline html-version"><a href="/zh-CN/docs/HTML/HTML5" class="page-not-created" title="This is a link to an unwritten page">HTML5</a></span>，或像素格式<span class="badge inline html-version">HTML 4.01</span>，或百分比格式指定 frame 的高度。默认值为<code>150</code>。
- high

	资源的加载优先级较高
- importance

	表示 <code>&lt;iframe&gt; </code>的 <code>src</code> 属性指定的资源的加载优先级。允许的值有：
 <dl>
  <dt id="auto"><code>auto</code> (default)</dt>
  <dd>不指定优先级。浏览器根据自身情况决定资源的加载顺序</dd>
  <dt id="high"><code>high</code></dt>
  <dd>资源的加载优先级较高</dd>
  <dt id="low"><code>low</code></dt>
  <dd>资源的加载优先级较低</dd>
 </dl>
 
- longdesc

	表示框架内容的长描述的 URL。由于广泛的误用，该属性对于无图形界面的浏览器不起作用。
- low

	资源的加载优先级较低
- marginheight

	这个属性定义了框架的内容距其上边框与下边框的距离，单位是像素。
- marginwidth

	这个属性定义了框架的内容距其左边框和右边框的距离，单位是像素。
- name

	用于定位嵌入的浏览上下文的名称。该名称可以用作 <a href="/zh-CN/docs/Web/HTML/Element/a"><code>&lt;a&gt;</code></a> 标签与 <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> 标签的 <code>target</code> 属性值，也可以用作 <a href="/zh-CN/docs/Web/HTML/Element/Input"><code>&lt;input&gt;</code></a> 标签和 <a href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a> 标签的 <code>formtarget</code> 属性值，还可以用作 <a href="/zh-CN/docs/Web/API/Window/open" title="window.open()"><code>window.open()</code></a> 方法的 <code>windowName</code> 参数值。
- referrerpolicy

	表示在获取 iframe&nbsp;资源时如何发送 <a href="/en-US/docs/Web/API/Document/referrer">referrer</a> 首部：
 <ul>
  <li><code>no-referrer</code>: 不发送 <a href="/zh-CN/docs/Web/HTTP/Headers/Referer"><code>Referer</code></a> 首部。</li>
  <li><code>no-referrer-when-downgrade</code>&nbsp;(default): 向不受 <a href="/zh-CN/docs/Glossary/TLS">TLS</a> (<a href="/zh-CN/docs/Glossary/https">HTTPS</a>) 保护的 <a href="/zh-CN/docs/Glossary/Origin">origin</a> 发送请求时，不发送 <a href="/zh-CN/docs/Web/HTTP/Headers/Referer"><code>Referer</code></a> 首部。</li>
  <li><code>origin</code>: referrer 首部中仅包含来源页面的源。换言之，仅包含来源页面的 <a href="/en-US/docs/Archive/Mozilla/URIScheme">scheme</a>, <a href="/zh-CN/docs/Glossary/Host">host</a>, 以及 <a href="/en-US/docs/Glossary/Port" class="only-in-en-us" title="Currently only available in English (US)">port <small>(en-US)</small></a>。</li>
  <li><code>origin-when-cross-origin</code>: 发起跨域请求时，仅在 referrer 中包含来源页面的源。发起同源请求时，仍然会在 referrer 中包含来源页面在服务器上的路径信息。</li>
  <li><code>same-origin</code>: 对于 <a href="/zh-CN/docs/Glossary/Same-origin_policy">same origin</a>（同源）请求，发送 referrer 首部，否则不发送。</li>
  <li><code>strict-origin</code>: 仅当被请求页面和来源页面具有相同的协议安全等级时才发送 referrer 首部（比如从采用 HTTPS 协议的页面请求另一个采用 HTTPS 协议的页面）。如果被请求页面的协议安全等级较低，则不会发送 referrer 首部（比如从采用 HTTPS 协议的页面请求采用 HTTP 协议的页面）。</li>
  <li><code>strict-origin-when-cross-origin</code>: 当发起同源请求时，在 referrer 首部中包含完整的 URL。当被请求页面与来源页面不同源但是有相同协议安全等级时（比如 HTTPS→HTTPS），在 referrer 首部中仅包含来源页面的源。当被请求页面的协议安全等级较低时（比如 HTTPS→HTTP），不发送 referrer 首部。</li>
  <li><code>unsafe-url</code>: 始终在 referrer 首部中包含源以及路径（但不包括 <a href="/en-US/docs/Web/API/HTMLHyperlinkElementUtils/hash">fragment</a>，<a href="/en-US/docs/Web/API/HTMLHyperlinkElementUtils/password">密码</a>，或<a href="/en-US/docs/Web/API/HTMLHyperlinkElementUtils/username">用户名</a>）。<strong>这个值是不安全的</strong>, 因为这样做会暴露受 TLS 保护的资源的源和路径信息。</li>
 </ul>
 
- sandbox

	该属性对呈现在 iframe 框架中的内容启用一些额外的限制条件。属性值可以为空字符串（这种情况下会启用所有限制），也可以是用空格分隔的一系列指定的字符串。有效的值有：
 <ul>
  <li><code>allow-downloads-without-user-activation</code> <abbr class="icon icon-experimental" title="Experimental. Expect behavior to change in the future.">
	<span class="visually-hidden">Experimental</span>
</abbr>: 允许在没有征求用户同意的情况下下载文件。</li>
  <li><code>allow-forms</code>: 允许嵌入的浏览上下文提交表单。如果没有使用该关键字，则无法提交表单。</li>
  <li><code>allow-modals</code>: 允许嵌入的浏览上下文打开模态窗口。</li>
  <li><code>allow-orientation-lock</code>: 允许嵌入的浏览上下文锁定屏幕方向（译者注：比如智能手机、平板电脑的水平朝向或垂直朝向）。</li>
  <li><code>allow-pointer-lock</code>: 允许嵌入的浏览上下文使用&nbsp;<a href="/zh-CN/docs/Web/API/Pointer_Lock_API">Pointer Lock API</a>.</li>
  <li><code>allow-popups</code>: 允许弹窗 (例如 window.open, target="_blank",&nbsp;<code>showModalDialog</code>)。如果没有使用该关键字，相应的功能将自动被禁用。</li>
  <li><code>allow-popups-to-escape-sandbox</code>: &nbsp;允许沙箱化的文档打开新窗口，并且新窗口不会继承沙箱标记。例如，安全地沙箱化一个广告页面，而不会在广告链接到的新页面中启用相同的限制条件。</li>
  <li><code>allow-presentation</code>: 允许嵌入的浏览上下文开始一个<a href="/en-US/docs/Web/API/PresentationRequest"> presentation session</a>。</li>
  <li><code>allow-same-origin</code>: 如果没有使用该关键字，嵌入的浏览上下文将被视为来自一个独立的源，这将使 <a href="/zh-CN/docs/Glossary/Same-origin_policy">same-origin policy</a> 同源检查失败。</li>
  <li><code>allow-scripts</code>: 允许嵌入的浏览上下文运行脚本（但不能创建弹窗）。如果没有使用该关键字，就无法运行脚本。</li>
  <li><code>allow-storage-access-by-user-activation</code> <abbr title="Experimental. Expect behavior to change in the future." class="icon icon-experimental">
	<span class="visually-hidden">Experimental</span>
</abbr>: 允许嵌入的浏览上下文通过 <a href="/en-US/docs/Web/API/Storage_Access_API">Storage Access API</a> 使用父级浏览上下文的存储功能。</li>
  <li><code>allow-top-navigation</code>: 允许嵌入的浏览上下文导航（加载）内容到顶级的浏览上下文。</li>
  <li><code>allow-top-navigation-by-user-activation</code>: 允许嵌入的浏览上下文<u><strong>在经过用户允许后</strong></u>导航（加载）内容到顶级的浏览上下文。</li>
 </ul>

 <div class="note notecard" id="sect7">
 <p><strong>注意：</strong></p>

 <ul>
  <li>当被嵌入的文档与主页面同源时，强烈建议不要同时使用 <code>allow-scripts</code> 和 <code>allow-same-origin</code>。如果同时使用，嵌入的文档就可以通过代码删除 <code>sandbox</code> 属性，如此，就安全性而言还不如不用<code>sandbox</code>。</li>
  <li>如果攻击者可以在沙箱化的 <code>iframe</code> 之外展示内容，例如用户在新标签页中打开内联框架，那么沙箱化也就没有意义了。建议把这种内容放置到独立的专用域中，以减小可能的损失。</li>
  <li>沙箱属性 (sandbox) 在 Internet Explorer 9 及更早的版本上不被支持。</li>
 </ul>
 </div>
 
- scrolling

	这个属性控制是否要在框架内显示滚动条，允许的值包括：
 <ul>
  <li><code>auto</code>: 仅当框架的内容超出框架的范围时显示滚动条。</li>
  <li><code>yes</code>: 始终显示滚动条。</li>
  <li><code>no</code>: 从不显示滚动条。</li>
 </ul>
 
- src

	被嵌套的页面的 URL 地址。使用 <code>about:blank</code> 值可以嵌入一个遵从<a href="/zh-CN/docs/Web/Security/Same-origin_policy">同源策略</a>的空白页。在 Firefox（version 65 及更高版本）、基于 Chromium 的浏览器、Safari/iOS 中使用代码移除 <code>iframe</code> 的 <code>src</code> 属性（例如通过 <a href="/zh-CN/docs/Web/API/Element/removeAttribute"><code>Element.removeAttribute()</code></a> ）会导致 <code>about:blank</code> 被载入 frame。
- srcdoc

	该属性是一段 HTML 代码，这些代码会被渲染到 iframe 中。如果浏览器不支持 <code>srcdoc</code> 属性，则会渲染 <code>src</code> 属性表示的内容。
- width

	以 CSS 像素格式<span class="badge inline html-version"><a class="page-not-created" href="/zh-CN/docs/HTML/HTML5" title="This is a link to an unwritten page">HTML5</a></span>，或以像素格式<span class="badge inline html-version">HTML 4.01</span>，或以百分比格式指定的 frame 的宽度。默认值是<code>300</code>。"#####
}

