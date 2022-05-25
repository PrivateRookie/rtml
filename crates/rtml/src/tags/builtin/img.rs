def! {
  img,
  Img,
  ImgArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<img>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/img)

---

**允许的内容**

无，它是一个 <a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>。

**是否可忽略关闭标签**

必须有开始标签，不可有结束标签。

**父标签**

接受嵌入内容的任意元素。

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLImageElement)


---

## 属性

 包含全局属性: true

- align

	图像相对于它周围上下文的对齐。使用 <a href="/zh-CN/docs/Web/CSS/float"><code>float</code></a> 和/或 <a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> 这两个 <a href="/zh-CN/docs/Glossary/CSS">CSS</a> 属性作为代替，而不是这个废弃的属性。允许的值：
 <dl>
  <dt id="top"><code>top</code></dt>
  <dd>等价于 <code style="white-space: nowrap;">vertical-align: top</code> 或 <code style="white-space: nowrap;">vertical-align: text-top</code></dd>
  <dt id="middle"><code>middle</code></dt>
  <dd>等价于 <code style="white-space: nowrap;">vertical-align: -moz-middle-with-baseline</code></dd>
  <dt id="bottom"><code>bottom</code></dt>
  <dd>默认值，等价于 <code style="white-space: nowrap;">vertical-align: unset</code> 或 <code style="white-space: nowrap;">vertical-align: initial</code></dd>
  <dt id="left"><code>left</code></dt>
  <dd>等价于 <code style="white-space: nowrap;">float: left</code></dd>
  <dt id="right"><code>right</code></dt>
  <dd>等价于 <code style="white-space: nowrap;">float: right</code></dd>
 </dl>
 
- alt

	定义了图像的备用文本描述。
 <div id="sect4" class="note notecard">
 <p><strong>注意：</strong>浏览器并非总是会显示图像。比如：</p>

 <ul>
  <li>非可视化浏览器（Non-visual browsers）（比如有视力障碍的人使用的音频浏览器）</li>
  <li>用户选择不显示图像（比如为了节省带宽，或出于隐私等考虑不加载包括图片在内的第三方资源文件）</li>
  <li>图像文件无效，或是使用了<a href="#supported_image_formats">不支持的格式</a></li>
 </ul>

 <p>在这些情况下，浏览器很可能会将图像替换为图像所属 <code>&lt;img&gt;</code> 元素的 <code>alt</code> 属性所提供的文本。基于上面罗列的原因，以及更多尚未列出的原因，建议尽可能地通过 <code>alt</code> 属性提供一些有用的信息。</p>
 </div>

 <p>如果省略 <code>alt</code> 属性，则表明该图像是内容的关键部分，但没有等效的文本可用。<br>
  如果把这个属性设置为空字符串（<code>alt=""</code>），则表明该图像<em>不是</em>内容的关键部分（这是一种装饰或者一个追踪像素点），非可视化浏览器在<a class="only-in-en-us" href="/en-US/docs/Glossary/Rendering_engine" title="Currently only available in English (US)">渲染 <small>(en-US)</small></a>的时候可能会忽略它。而且，如果图片加载失败，可视化浏览器会隐藏表示图片损坏的图标。</p>

 <p>将图像复制并粘贴为文本，或是将图像的链接保存为浏览器书签时，也会用到此属性。</p>
 
- anonymous

	执行一个跨域请求（比如，有 <a href="/zh-CN/docs/Web/HTTP/Headers/Origin"><code>Origin</code></a> <a href="/zh-CN/docs/Glossary/HTTP">HTTP</a> header）。但是没有发送证书（比如，没有 cookie，没有 <a class="external" href="https://tools.ietf.org/html/rfc5280" rel=" noopener">X.509 证书</a>，没有 <a href="/zh-CN/docs/Web/HTTP/Authentication#basic_authentication_scheme">HTTP 基本授权认证</a>）。如果服务器没有把证书给到源站（没有设置 <a href="/zh-CN/docs/Web/HTTP/Headers/Access-Control-Allow-Origin"><code>Access-Control-Allow-Origin</code></a> HTTP 头），图像会被污染，而且它的使用会被限制。
- async

	异步解码图像，以减少其他内容的显示延迟。
- auto

	<strong>不指定优先级。</strong>浏览器可以使用自己的算法来为图像选择优先级。
- border

	插入到图像的左侧和右侧的空白像素的值。使用 CSS 属性 <a href="/zh-CN/docs/Web/CSS/margin"><code>margin</code></a> 代替此废弃属性。
- bottom

	等价于 <code style="white-space: nowrap;">float: left</code>
- crossorigin

	这个枚举属性表明是否必须使用 CORS 完成相关图像的抓取。<a href="/zh-CN/docs/Web/HTML/CORS_enabled_image">启用 CORS 的图像</a> 可以在 <a href="/zh-CN/docs/Web/HTML/Element/canvas"><code>&lt;canvas&gt;</code></a> 元素中重复使用，而不会被<a href="/zh-CN/docs/Web/HTML/CORS_enabled_image#what_is_a_tainted_canvas">污染</a>（tainted）。允许的值有：
 <dl>
  <dt id="anonymous"><code>anonymous</code></dt>
  <dd>执行一个跨域请求（比如，有 <a href="/zh-CN/docs/Web/HTTP/Headers/Origin"><code>Origin</code></a> <a href="/zh-CN/docs/Glossary/HTTP">HTTP</a> header）。但是没有发送证书（比如，没有 cookie，没有 <a class="external" href="https://tools.ietf.org/html/rfc5280" rel=" noopener">X.509 证书</a>，没有 <a href="/zh-CN/docs/Web/HTTP/Authentication#basic_authentication_scheme">HTTP 基本授权认证</a>）。如果服务器没有把证书给到源站（没有设置 <a href="/zh-CN/docs/Web/HTTP/Headers/Access-Control-Allow-Origin"><code>Access-Control-Allow-Origin</code></a> HTTP 头），图像会被污染，而且它的使用会被限制。</dd>
  <dt id="use-credentials"><code>use-credentials</code></dt>
  <dd>一个有证书的跨域请求（比如，有 <code>Origin</code> HTTP header）被发送（比如，cookie，一份证书，或者 HTTP 基本验证信息）。如果服务器没有给源站发送证书（通过 <code>Access-Control-Allow-Credentials</code> HTTP header），图像将会被污染，且它的使用会受限制。</dd>
 </dl>
 当用户并未显式使用本属性时，默认不使用 CORS 发起请求（例如，不会向服务器发送<code>原有的</code>HTTP 头部信息），可防止其在 <a href="/zh-CN/docs/Web/HTML/Element/canvas"><code>&lt;canvas&gt;</code></a> 中的使用。如果无效，默认当做 <code>anonymous</code> 关键字生效。更多信息，请查看 <a href="/zh-CN/docs/Web/HTML/Attributes/crossorigin">CORS 属性设置</a> 。
- decoding

	
 <p>为浏览器提供图像解码方式上的提示。允许的值：</p>

 <dl>
  <dt id="sync"><code>sync</code></dt>
  <dd>同步解码图像，实现与其他内容的显示相互斥的原子显示。
  <div class="standardNoteBlock" id="sect5">
  <p><strong>译者注：</strong>这里的原文是：</p>

  <p>Decode the image synchronously, for atomic presentation with other content.</p>

  <p>此图像的解码将是一个原子操作，在完成解码显示之前，不被其他内容的显示而打断，因此其他内容的显示会被延迟。</p>
  </div>
  </dd>
  <dt id="async"><code>async</code></dt>
  <dd>异步解码图像，以减少其他内容的显示延迟。</dd>
  <dt id="auto"><code>auto</code></dt>
  <dd>默认值：不指定解码方式，由浏览器决定哪一种对用户来说是最合适的。</dd>
 </dl>
 
- eager

	立即加载图像，不管它是否在可视视口（visible viewport）之外（默认值）。
- height

	图像的高度，在 <span class="badge inline html-version"><a href="/zh-CN/docs/HTML/HTML5" title="This is a link to an unwritten page" class="page-not-created">HTML5</a></span> 中的单位是 CSS 像素，在 <span class="badge inline html-version"><a href="/zh-CN/docs/Web/HTML">HTML 4</a></span> 中既可以是像素，也可以是百分比。可以只指定 <code>width</code> 和 <code>height</code> 中的一个值，浏览器会根据原始图像进行缩放。
- high

	此图像在下载时优先级<strong>较高</strong>。
- hspace

	一个指向更详细的图像描述的链接。可能的值是一个 <a href="/zh-CN/docs/Glossary/URL">URL</a> 或一个页面上其他元素的 <a href="/zh-CN/docs/Web/HTML/Global_attributes#attr-id"><code>id</code></a>。
 <div class="note notecard" id="sect8">
 <p><strong>备注：</strong> 此属性的当前最新的 <a href="/en-US/docs/Glossary/W3C" title="Currently only available in English (US)" class="only-in-en-us">W3C <small>(en-US)</small></a> 版本，<a rel=" noopener" class="external" href="https://www.w3.org/TR/html52/obsolete.html#element-attrdef-img-longdesc">HTML 5.2</a> 中被提到，但在 <a href="/zh-CN/docs/Glossary/WHATWG">WHATWG</a> 组织的 <a href="https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element" rel=" noopener" class="external">HTML Living Standard</a> 中依然处于被移除的状态。它的未来尚无定数；authors should use a <a title="Currently only available in English (US)" href="/en-US/docs/Glossary/WAI" class="only-in-en-us">WAI <small>(en-US)</small></a>-<a href="/zh-CN/docs/Glossary/ARIA">ARIA</a> alternative such as <a href="https://www.w3.org/TR/wai-aria-1.1/#aria-describedby" class="external" rel=" noopener"><code>aria-describedby</code></a> or <a rel=" noopener" class="external" href="https://www.w3.org/TR/wai-aria-1.1/#aria-details"><code>aria-details</code></a>.</p>
 </div>
 
- importance

	指示下载资源时相对重要性，或者说优先级。允许的值：
 <dl>
  <dt id="auto_2"><code>auto</code></dt>
  <dd><strong>不指定优先级。</strong>浏览器可以使用自己的算法来为图像选择优先级。</dd>
  <dt id="high"><code>high</code></dt>
  <dd>此图像在下载时优先级<strong>较高</strong>。</dd>
  <dt id="low"><code>low</code></dt>
  <dd>此图像在下载时优先级<strong>较低</strong>。</dd>
 </dl>
 
- intrinsicsize

	This attribute tells the browser to ignore the actual <a href="/en-US/docs/Glossary/Intrinsic_Size" title="Currently only available in English (US)" class="only-in-en-us">intrinsic size <small>(en-US)</small></a> of the image and pretend it’s the size specified in the attribute. Specifically, the image would raster at these dimensions and <code>naturalWidth</code>/<code>naturalHeight</code> on images would return the values specified in this attribute. <a rel=" noopener" class="external" href="https://github.com/ojanvafai/intrinsicsize-attribute">Explainer</a>, <a class="external" href="https://googlechrome.github.io/samples/intrinsic-size/index.html" rel=" noopener">examples</a>
- ismap

	这个布尔属性表示图像是否是<a href="https://en.wikipedia.org/wiki/Image_map#Server-side" rel=" noopener" class="external">服务器端 map</a> 的一部分。如果是，那么点击图片的精准坐标将会被发送到服务器。
 <div class="note notecard" id="sect6">
 <p><strong>使用说明：</strong>只有在 <code>&lt;img&gt;</code> 元素是一个拥有有效 <a href="/zh-CN/docs/Web/HTML/Element/a#attr-href"><code>href</code></a> 属性的 <a href="/zh-CN/docs/Web/HTML/Element/a"><code>&lt;a&gt;</code></a> 元素的后代元素的情况下，这个属性才会被允许使用。</p>
 </div>
 
- lazy

	延迟加载图像，直到它和视口接近到一个计算得到的距离，由浏览器定义。
- left

	等价于 <code style="white-space: nowrap;">float: right</code>
- loading

	指示浏览器应当如何加载该图像。允许的值：
 <dl>
  <dt id="eager"><code>eager</code></dt>
  <dd>立即加载图像，不管它是否在可视视口（visible viewport）之外（默认值）。</dd>
  <dt id="lazy"><code>lazy</code></dt>
  <dd>延迟加载图像，直到它和视口接近到一个计算得到的距离，由浏览器定义。</dd>
 </dl>
 
- longdesc

	元素的名字。使用 <a href="/zh-CN/docs/Web/HTML/Global_attributes#attr-id"><code>id</code></a> 属性代替此废弃属性。
- low

	此图像在下载时优先级<strong>较低</strong>。
- middle

	默认值，等价于 <code style="white-space: nowrap;">vertical-align: unset</code> 或 <code style="white-space: nowrap;">vertical-align: initial</code>
- name

	插入到图像的上方和下方的空白像素的数组。使用 CSS 属性 <a href="/zh-CN/docs/Web/CSS/margin"><code>margin</code></a> 代替此废弃属性。
- referrerpolicy

	A string indicating which referrer to use when fetching the resource:
 <ul>
  <li><code>no-referrer</code>: The <a href="/zh-CN/docs/Web/HTTP/Headers/Referer"><code>Referer</code></a> header will not be sent.</li>
  <li><code>no-referrer-when-downgrade</code>: No <code>Referer</code> header is sent when navigating to an origin without <a href="/zh-CN/docs/Glossary/https">HTTPS</a>. This is the default if no policy is otherwise specified.</li>
  <li><code>origin</code>: The <code>Referer</code> header will include the page's origin (<a class="page-not-created" title="此页面仍未被本地化, 期待您的翻译!">scheme</a>, <a href="/zh-CN/docs/Glossary/Host">host</a>, and <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Glossary/Port">port <small>(en-US)</small></a>).</li>
  <li><code>origin-when-cross-origin</code>: Navigating to other origins will limit the included referral data to the scheme, host, and port, while navigating from the same origin will include the full path and query string.</li>
  <li><code>unsafe-url</code>: The <code>Referer</code> header will always include the origin, path and query string, but not the fragment, password, or username. <strong>This is unsafe</strong> because it can leak information from TLS-protected resources to insecure origins.</li>
 </ul>
 
- right

	图像周围的边框宽度。使用 <a href="/zh-CN/docs/Glossary/CSS">CSS</a> 属性 <a href="/zh-CN/docs/Web/CSS/border"><code>border</code></a> 代替此废弃属性。
- sizes

	表示资源大小的、以逗号隔开的一个或多个字符串。每一个资源大小包括：
 <ol>
  <li>一个<a href="/zh-CN/docs/Web/CSS/Media_Queries/Using_media_queries#syntax">媒体条件</a>。最后一项一定是被忽略的。</li>
  <li>一个资源尺寸的值。</li>
 </ol>

 <p>Media Conditions describe properties of the <em>viewport</em>, not of the <em>image</em>. For example, <code>(max-height: 500px) 1000px</code> proposes to use a source of 1000px width, if the <em>viewport </em>is not higher than 500px.</p>

 <p>资源尺寸的值被用来指定图像的预期尺寸。当 <code>srcset</code> 中的资源使用了宽度描述符 <code>w</code> 时，<a href="/zh-CN/docs/Glossary/User_agent">用户代理</a>会使用当前图像大小来选择 <code>srcset</code> 中合适的一个图像 URL。被选中的尺寸影响图像的<a class="only-in-en-us" href="/en-US/docs/Glossary/Intrinsic_Size" title="Currently only available in English (US)">显示大小 <small>(en-US)</small></a>（如果没有影响大小的 <a href="/zh-CN/docs/Glossary/CSS">CSS</a> 样式被应用的话）。如果没有设置 <code>srcset</code> 属性，或者没有属性值，那么 <code>sizes</code> 属性也将不起作用。</p>
 
- src

	图像的 <a href="/zh-CN/docs/Glossary/URL">URL</a>，这个属性对 <code>&lt;img&gt;</code> 元素来说是必需的。在支持 <code>srcset</code> 的浏览器中，<code>src</code> 被当做拥有一个像素密度的描述符 <code>1x</code> 的候选图像处理，除非一个图像拥有这个像素密度描述符已经被在 <code>srcset</code> 或者 <code>srcset</code> 包含 <code>w</code> 描述符中定义了。
- srcset

	以逗号分隔的一个或多个字符串列表表明一系列用户代理使用的可能的图像。每一个字符串由以下组成：
 <ol>
  <li>指向图像的 <a href="/zh-CN/docs/Glossary/URL">URL</a>。</li>
  <li>可选地，再加一个空格之后，附加以下的其一：
   <ul>
	<li>一个宽度描述符，这是一个正整数，后面紧跟 '<code>w</code>' 符号。该整数宽度除以 sizes 属性给出的资源（source）大小来计算得到有效的像素密度，即换算成和 x 描述符等价的值。</li>
	<li>一个像素密度描述符，这是一个正浮点数，后面紧跟 '<code>x</code>' 符号。</li>
   </ul>
  </li>
 </ol>

 <p>如果没有指定源描述符，那它会被指定为默认的 <code>1x</code>。</p>

 <p>在相同的 <code>srcset</code> 属性中混合使用宽度描述符和像素密度描述符时，会导致该值无效。重复的描述符（比如，两个源在相同的 <code>srcset</code> 两个源都是 <code>2x</code>）也是无效的。</p>

 <p>The user agent selects any of the available sources at its discretion. This provides them with significant leeway to tailor their selection based on things like user preferences or <a href="/zh-CN/docs/Glossary/Bandwidth">bandwidth</a> conditions. See our <a href="/zh-CN/docs/Learn/HTML/Multimedia_and_embedding/Responsive_images">Responsive images</a> tutorial for an example.</p>
 
- sync

	同步解码图像，实现与其他内容的显示相互斥的原子显示。
  <div class="standardNoteBlock" id="sect5">
  <p><strong>译者注：</strong>这里的原文是：</p>

  <p>Decode the image synchronously, for atomic presentation with other content.</p>

  <p>此图像的解码将是一个原子操作，在完成解码显示之前，不被其他内容的显示而打断，因此其他内容的显示会被延迟。</p>
  </div>
  
- top

	等价于 <code style="white-space: nowrap;">vertical-align: -moz-middle-with-baseline</code>
- use-credentials

	一个有证书的跨域请求（比如，有 <code>Origin</code> HTTP header）被发送（比如，cookie，一份证书，或者 HTTP 基本验证信息）。如果服务器没有给源站发送证书（通过 <code>Access-Control-Allow-Credentials</code> HTTP header），图像将会被污染，且它的使用会受限制。
- usemap

	与元素相关联的 <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/map" class="only-in-en-us">image map (en-US)</a> 的部分 URL（以 '#' 开始的部分）。
 <div id="sect7" class="note notecard">
 <p><strong>使用说明： </strong>如果 <code>&lt;img&gt;</code> 元素是 <a href="/zh-CN/docs/Web/HTML/Element/a"><code>&lt;a&gt;</code></a> 或 <a href="/zh-CN/docs/Web/HTML/Element/button"><code>&lt;button&gt;</code></a> 元素的后代元素则不能使用这个属性。</p>
 </div>
 
- vertical-align

	等价于 <code style="white-space: nowrap;">vertical-align: top</code> 或 <code style="white-space: nowrap;">vertical-align: text-top</code>
- width

	图像的宽度，在 <span class="badge inline html-version"><a href="/zh-CN/docs/HTML/HTML5" class="page-not-created" title="This is a link to an unwritten page">HTML5</a></span> 中单位是 CSS 像素， 在 <span class="badge inline html-version"><a href="/zh-CN/docs/Web/HTML">HTML 4</a></span> 中可以是像素也可以是百分比。"#####
}
