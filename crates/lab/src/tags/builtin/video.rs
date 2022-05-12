def! {
  video,
  Video,
  VideoArg,
  doc:
  "zh-CN" = r#####"`<video>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/video)

---

**允许的内容**



**是否可忽略关闭标签**



**父标签**



[Dom API]()


---

## 属性

 包含全局属性: true

- <a rel=" noopener" href="https://wicg.github.io/picture-in-picture/#disable-pip" class="external">disablePictureInPicture</a>

	跨域请求 A cross-origin request (i.e. with <code>Origin:</code> HTTP header) 会被执行，且凭证会被发送 (即， 发送一个 cookie，一个证书和 HTTP Basic 授权会被执行)。如果服务器不提供证书给源站点 (通过<code>Access-Control-Allow-Credentials:</code> HTTP 头)，图像会被 <em>污染</em> 且它的使用会受限。
- autobuffer

	布尔属性；指定后，视频会自动开始缓存，即使没有设置自动播放。该属性适用于视频被认为可能会播放（比如，用户导航到专门播放视频的页面，而不是那种嵌入视频还有其它内容的页面）。视频会一直缓存到媒体缓存满。
    <div id="sect5" class="note notecard"><strong>实现备注： </strong> 虽然是 HTML5 规范的早期草案的一部分， <code>autobuffer</code> 属性在稍晚的版本被去掉了。 Gecko 2.0 和其它浏览器中已经移除了这个属性，而且有些浏览器中从未实现。规范定义了一个新的枚举属性， <code>preload</code>， 用不同的语法来取代 <code>autobuffer</code> 属性。 <a class="external" href="https://bugzilla.mozilla.org/show_bug.cgi?id=548523" rel=" noopener">bug&nbsp;548523</a></div>
 
- autoplay

	布尔属性；指定后，视频会马上自动开始播放，不会停下来等着数据载入结束。
- buffered

	这个属性可以读取到哪段时间范围内的媒体被缓存了。该属性包含了一个 <a href="/zh-CN/docs/Web/API/TimeRanges"><code>TimeRanges</code></a> 对象。
- controls

	加上这个属性，Gecko 会提供用户控制，允许用户控制视频的播放，包括音量，跨帧，暂停/恢复播放。
- controlslist

	
 <p>当浏览器显示自己的控件集 (例如，当指定了 Controls 属性时)，Controlslist 属性将帮助浏览器选择在媒体元素上显示的控件。</p>
 <p>允许接受的 value 有<code>nodownload</code>,<code>nofullscreen</code>和<code>noremoteplayback</code></p>
 <div id="sect6" class="note notecard">
    <p>如果要禁用图片模式 (和控件)，请使用<code>disablePictureInPicture</code>属性。</p>
 </div>
 
- crossorigin

	该枚举属性指明抓取相关图片是否必须用到 CORS（跨域资源共享）。 <a href="/zh-CN/docs/Web/HTML/CORS_enabled_image" title="CORS_Enabled_Image">支持 CORS 的资源</a> 可在 <a href="/zh-CN/docs/Web/HTML/Element/canvas"><code>&lt;canvas&gt;</code></a> 元素中被重用，而不会被<em>污染</em>。允许的值如下：
    <dl>
        <dt id="anonymous">anonymous</dt>
        <dd>跨域请求（即，使用 <code>Origin:</code> 的 HTTP 头）会被执行。但是不发送凭证（即，不发送 cookie， X.509 证书或者 HTTP Basic 授权）。如果服务器不提供证书给源站点 (不设置 <code>Access-Control-Allow-Origin:</code> HTTP 头)，图片会被 <em>污染</em> 并且它的使用会受限。</dd>
        <dt id="use-credentials">use-credentials</dt>
        <dd>跨域请求 A cross-origin request (i.e. with <code>Origin:</code> HTTP header) 会被执行，且凭证会被发送 (即， 发送一个 cookie，一个证书和 HTTP Basic 授权会被执行)。如果服务器不提供证书给源站点 (通过<code>Access-Control-Allow-Credentials:</code> HTTP 头)，图像会被 <em>污染</em> 且它的使用会受限。</dd>
    </dl>
    不加这个属性时，抓取资源不会走 CORS 请求 (即，不会发送 <code>Origin:</code> HTTP 头)，保证其在 <a href="/zh-CN/docs/Web/HTML/Element/canvas"><code>&lt;canvas&gt;</code></a> 元素中使用时不会被污染。如果指定非法值，会被当作指定了枚举关键字 <strong>anonymous</strong> 一样使用。 查看 <a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Attributes/crossorigin">CORS 设置属性 (en-US)</a> 获取更多信息。
- currentTime

	跨域请求（即，使用 <code>Origin:</code> 的 HTTP 头）会被执行。但是不发送凭证（即，不发送 cookie， X.509 证书或者 HTTP Basic 授权）。如果服务器不提供证书给源站点 (不设置 <code>Access-Control-Allow-Origin:</code> HTTP 头)，图片会被 <em>污染</em> 并且它的使用会受限。
- disableRemotePlayback
    <p>读取<code>CurentTime</code>返回一个双精度浮点值，指示以秒为单位的媒体的当前播放位置。如果 video 尚未开始播放，则会在开始播放后返回偏移量。通过<code>CurentTime</code>将当前播放位置设置为给定时间，会在加载媒体时将媒体查找到该位置（从指定的位置开始播放）。</p>
    <p>媒体正在播放的情况下，如果媒体缓冲区的数据已经过期（视频已经播放完），则&nbsp;<a href="/zh-CN/docs/Glossary/User_agent">user agent</a>有可能无法正常拿到数据。有些媒体可能有一个不以 0 秒开始的媒体时间线（不是从头开始播放的），因此应该将<code>currentTime</code>的时间设置在其数据失效之前。<a class="page-not-created" title="此页面仍未被本地化, 期待您的翻译!"><code>getStartDate()</code></a> 这个方法可以用来确定媒体时间线起始的坐标。</p>
 
- duration
    <p>防止浏览器建议图片中的上下文菜单或在某些情况下自动请求图片中的图片。该属性可以禁用 <code>video</code> 元素的画中画特性，右键菜单中的 “画中画” 选项会被禁用</p>
 
- height
    一个布尔属性，用于禁用使用有线连接的设备 (HDMI、DVI 等) 的远程播放功能。无线技术 (Miracast、Chromecast、DLNA、AirPlay 等)。
- intrinsicsize

	一个双精度浮点值，它指示媒体的持续时间 (总长度)，以秒为单位，在媒体的时间线上。如果元素上没有媒体，或者媒体无效，则返回的值为 NaN。如果媒体没有已知终点 (例如时间未知的实时流、网络广播、来自 WebRTC 的媒体等等)，那么这个值就是 Infinity。
- loop

	视频展示区域的高度，单位是 CSS 像素。
- muted

    <p>这个属性告诉浏览器忽略图像的实际内在大小，并假装它是属性中指定的大小。具体来说，图像将在这些维度上展开，图像上的<code>naturalWidth</code>/<code>naturalHeight</code>&nbsp;将返回此属性中指定的值。<a rel=" noopener" href="https://github.com/WICG/intrinsicsize-attribute" class="external">Explainer</a>，<a href="https://googlechrome.github.io/samples/intrinsic-size/index.html" class="external" rel=" noopener">examples</a></p>
 
- played

	布尔属性，指明了视频里的音频的默认设置。设置后，音频会初始化为静音。默认值是 false，意味着视频播放的时候音频也会播放 。
- playsinline

	布尔属性；指定后，会在视频结尾的地方，自动返回视频开始的地方。
- poster

	一个 <a href="/zh-CN/docs/Web/API/TimeRanges"><code>TimeRanges</code></a> 对象，指明了视频已经播放的所有范围。
- preload

	
 <p>一个布尔属性，标志视频将被 “inline” 播放，即在元素的播放区域内。请注意，没有此属性并不意味着视频始终是全屏播放的。</p>
 
- src

	该枚举属性旨在告诉浏览器作者认为达到最佳的用户体验的方式是什么。可能是下列值之一：
    <ul>
        <li>none: 提示作者认为用户不需要查看该视频，服务器也想要最小化访问流量；换句话说就是提示浏览器该视频不需要缓存。</li>
        <li>metadata: 提示尽管作者认为用户不需要查看该视频，不过抓取元数据（比如：长度）还是很合理的。</li>
        <li>auto: 用户需要这个视频优先加载；换句话说就是提示：如果需要的话，可以下载整个视频，即使用户并不一定会用它。</li>
        <li><em>空字符串</em>：也就代指 auto 值。</li>
    </ul>

    <p>假如不设置，默认值就是浏览器定义的了（即，不同浏览器会选择自己的默认值），即使规范建议设置为 metadata。</p>

    <div class="note notecard" id="sect9"><strong>使用备注：</strong>
        <ul>
            <li><code>autoplay</code> 属性优先于 <code>preload</code> 假如用户想自动播放视频，那么很明显浏览器需要下载视频。同时设置<code>autoplay</code> 和 <code>preload</code>属性在规范里是允许的。</li>
            <li>规范没有强制浏览器去遵循该属性的值；这仅仅只是个提示。</li>
        </ul>
    </div>
 
- width

	一个海报帧的 URL，用于在用户播放或者跳帧之前展示。如果属性未指定，那么在第一帧可用之前什么都不会展示；之后第一帧就像海报帧一样展示。"#####
}

