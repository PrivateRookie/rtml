def! {
  audio,
  Audio,
  AudioArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<audio>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/audio)

---

**允许的内容**



**是否可忽略关闭标签**



**父标签**



[Dom API]()


---

## 属性

 包含全局属性: true

- HTMLMediaElement.audioTracks

	一个&nbsp;<a title="Currently only available in English (US)" href="/en-US/docs/Web/API/AudioTrackList" class="only-in-en-us"><code>AudioTrackList</code> <small>(en-US)</small></a> 包含所有的媒体对象的音轨。你能在为&nbsp;<code>addtrack</code>&nbsp;事件添加监听，以在新音轨添加进元素时获得通知。
- HTMLMediaElement.textTracks

	在该&nbsp;<a title="Currently only available in English (US)" class="only-in-en-us" href="/en-US/docs/Web/API/TextTrackList"><code>TextTrackList</code> <small>(en-US)</small></a> 对象上添加监听，以在文字轨道被添加进元素时获得通知。（也许用于字幕，译者猜测）
- HTMLMediaElement.videoTracks

	在该&nbsp;<a href="/zh-CN/docs/Web/API/VideoTrackList"><code>VideoTrackList</code></a> 对象上添加监听，以在视频轨道被添加进元素时获得通知。
- anonymous

	在发送跨域请求时不携带验证信息。换句话说，浏览器在发送<code>Origin:</code> HTTP 请求首部时将不携带 cookie、 X.509 安全令牌、也不会执行任何 HTTP 基本认证。如果服务器没有给予源站信任（也就是说没有设置 <code>Access-Control-Allow-Origin:</code> 响应首部），那么图片就被认为是污染的，它就会被限制使用。
- autoplay

	布尔值属性；声明该属性，音频会尽快自动播放，不会等待整个音频文件下载完成。
 <div id="sect1" class="note notecard">
 <p><strong>注意：</strong>自动播放音频（或有声视频）可能会破坏用户体验，所以应该尽可能避免。如果你一定要提供自动播放功能，你应该加入开关（让用户主动打开自动播放）。然而，如果需要创建一些媒体元素，其播放源由用户在稍后设置，自动播放就会很有用。想了解如果正确使用自动播放，可参见我们的<a href="/zh-CN/docs/Web/Media/Autoplay_guide">自动播放指南</a>。</p>
 </div>
 
- controls

	如果声明了该属性，浏览器将提供一个包含声音，播放进度，播放暂停的控制面板，让用户可以控制音频的播放。
- crossorigin

	枚举属性&nbsp; 展示音频资源是否可以通过 CORS 加载。<a href="/zh-CN/docs/Web/HTML/CORS_enabled_image">支持 CORS 的资源&nbsp;</a>&nbsp;可以被 <a href="/zh-CN/docs/Web/HTML/Element/canvas"><code>&lt;canvas&gt;</code></a> 元素复用而不污染。可选值如下：
 <dl>
  <dt id="anonymous"><code>anonymous</code></dt>
  <dd>在发送跨域请求时不携带验证信息。换句话说，浏览器在发送<code>Origin:</code> HTTP 请求首部时将不携带 cookie、 X.509 安全令牌、也不会执行任何 HTTP 基本认证。如果服务器没有给予源站信任（也就是说没有设置 <code>Access-Control-Allow-Origin:</code> 响应首部），那么图片就被认为是污染的，它就会被限制使用。</dd>
  <dt id="use-credentials"><code>use-credentials</code></dt>
  <dd>在发送跨域请求时携带验证信息。换句话说，在发送<code>Origin:</code> HTTP 请求首部时将携带 cookie、安全令牌、并且执行 HTTP 基本认证。如果服务器没有给予源站信任（通过设置<code>Access-Control-Allow-Credentials:</code> 响应首部）那么图片就被认为是污染的，它就会被限制使用。</dd>
 </dl>
 在未指定时，资源将不通过 CORS 请求来获取（也就是不发送 <code>Origin: </code>请求首部），以保护 <a href="/zh-CN/docs/Web/HTML/Element/canvas"><code>&lt;canvas&gt;</code></a>&nbsp; 元素中未污染的内容。如果验证失败，它会表现的好像&nbsp;<strong>anonymous&nbsp;</strong>选项是选中的。查看&nbsp;<a href="/en-US/docs/Web/HTML/Attributes/crossorigin" class="only-in-en-us" title="Currently only available in English (US)">CORS settings attributes (en-US)</a>&nbsp;来获取更多信息。
- currentTime

	
 <p>读取&nbsp;<code>currentTime</code>&nbsp;属性将返回一个双精度浮点值，用以标明以秒为单位的当前音频的播放位置。如果音频的元数据暂时无法访问——这意味着你无法的知道媒体的开始或持续时间。这时，<code>currentTime</code>&nbsp;相对应的，能够被用于改变重播的时间。否则，设置&nbsp;<code>currentTime</code>&nbsp;将设置当前的播放位置，并且会自动搜寻到媒体的那个位置，如果媒体目前已经被加载的话。</p>

 <p>如果音频是以流的形式加载的，并且数据超出了媒体的缓冲区（buffer），<a href="/zh-CN/docs/Glossary/User_agent">user agent</a> 可能无法获取资源的某些部分。另一些音频的时间轴可能并非从 0 秒开始，所以设置&nbsp;<code>currentTime</code>&nbsp;到一个开始时间之前的时间可能会失败。举个例子，如果音频媒体的时间轴从 12 小时开始，把&nbsp;<code>currentTime</code>&nbsp;设置到 3600 将会尝试把当前播放位置设置到媒体的开始位置之前，从而导致错误。<a class="page-not-created" title="此页面仍未被本地化, 期待您的翻译!"><code>getStartDate()</code></a> 方法能够用于确定媒体时间轴的开始位置。</p>
 
- disableRemotePlayback

	这是一个布尔值，用来禁用在远程设备上进行进度控制的能力。这些设备通过有线（比如 HDMI, DVI）或无线技术（比如 Miracast, Chromecast, DLNA, AirPlay,）来与 web 连接。请参考&nbsp;&nbsp;<a href="https://www.w3.org/TR/remote-playback/#the-disableremoteplayback-attribute" class="external" rel=" noopener">this proposed specification</a>&nbsp;来获取更多信息。
 <div id="sect2" class="note notecard">
 <p>在 Safari 中，你能使用&nbsp;<a class="external" href="https://developer.apple.com/library/archive/documentation/AudioVideo/Conceptual/AirPlayGuide/OptingInorOutofAirPlay/OptingInorOutofAirPlay.html" rel=" noopener"><code>x-webkit-airplay="deny"</code></a>&nbsp;作为兜底方案。</p>
 </div>
 
- duration

	这是一个双精度浮点数，指明了音频在时间轴中的持续时间（总长度），以秒为单位。如果元素上没有媒体，或者媒体是不可用的，那么会返回&nbsp;<code>NaN</code>。如果媒体找不到确切的结尾（比如不确定长度的直播流，网络电台，或者是通过 <a href="/zh-CN/docs/Web/API/WebRTC_API">WebRTC</a>&nbsp;连接的流），那么这个值将返回&nbsp;<code>+Infinity</code>。
- loop

	布尔属性；如果声明该属性，将循环播放音频。
- muted

	表示是否静音的布尔值。默认值为 <code>false</code>，表示有声音。
- preload

	枚举属性，让开发者自行思考来示意浏览器使用何种加载方式以达到最好的用户体验。可以是以下属性之一：
 <ul>
  <li><code>none</code>: 示意用户可能不会播放该音频，或者服务器希望节省带宽；换句话说，该音频不会被缓存；</li>
  <li><code>metadata</code>: 示意即使用户可能不会播放该音频，但获取元数据&nbsp;(例如音频长度) 还是有必要的。</li>
  <li><code>auto</code>: 示意用户可能会播放音频；换句话说，如果有必要，整个音频都将被加载，即使用户不期望使用。</li>
  <li><em>空字符串</em> : 等效于<code>auto</code>属性。</li>
 </ul>

 <p>不同浏览器会有自己的默认值，规范建议的默认值为&nbsp;<code>metadata</code>.</p>

 <div id="sect3" class="note notecard">
 <p><strong>使用说明：</strong></p>

 <ul>
  <li><code>autoplay</code>&nbsp;属性的优先级高于&nbsp;<code>preload</code>。如果&nbsp;<code>autoplay</code>&nbsp;被指定，浏览器将显式地开始下载媒体以供播放。</li>
  <li>浏览器并不被强制遵循该属性的规范，该属性只是一个建议与提示。</li>
 </ul>
 </div>
 
- src

	嵌入的音频的 URL。 该 URL 应遵从&nbsp;<a href="/en-US/docs/Web/HTTP/CORS">HTTP access controls</a>. 这是一个可选属性；你可以在 audio 元素中使用&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/source"><code>&lt;source&gt;</code></a> 元素来替代该属性指定嵌入的音频。
- use-credentials

	在发送跨域请求时携带验证信息。换句话说，在发送<code>Origin:</code> HTTP 请求首部时将携带 cookie、安全令牌、并且执行 HTTP 基本认证。如果服务器没有给予源站信任（通过设置<code>Access-Control-Allow-Credentials:</code> 响应首部）那么图片就被认为是污染的，它就会被限制使用。"#####
}
