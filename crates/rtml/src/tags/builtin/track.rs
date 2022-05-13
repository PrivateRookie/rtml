def! {
  track,
  Track,
  TrackArg,
  doc:
  "zh-CN" = r#####"`<track>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/track)

---

**允许的内容**



**是否可忽略关闭标签**



**父标签**



[Dom API]()


---

## 属性

 包含全局属性: true

- default

	该属性定义了该 track 应该启用，除非用户首选项指定了更合适一个 track。每个媒体元素里面只有一个 <code>track</code> 元素可以有这个属性。
- kind

	定义了 text track 应该如何使用。如果省略了该属性，默认的 kind 值就是 <code>subtitles</code>。下面是允许的关键字：
 <ul>
  <li><code>subtitles</code>
   <ul>
	<li>字幕给观影者看不懂的内容提供了翻译。比如英文电影里非英文的对话框或者文字。</li>
	<li>字幕可能包含额外的内容，通常有附加的背景信息。比如在电影星球大战开头的文字，或者某个场景的日期，时间，还有地点。</li>
   </ul>
  </li>
  <li>captions
   <ul>
	<li>隐藏式字幕提供了音频的转录甚至是翻译。</li>
	<li>可能包含重要的非言语的信息，比如音乐提示或者音效。可以指定提示音的源文件 (e.g. music, text, character).</li>
	<li>适用于耳聋的用户或者当调成静音的时候。</li>
   </ul>
  </li>
  <li><code>descriptions</code>
   <ul>
	<li>视频内容的文本描述。</li>
	<li>适用于失明用户或者当视频不可见的场景。</li>
   </ul>
  </li>
  <li><code>chapters</code>
   <ul>
	<li>章节标题用于用户浏览媒体资源的时候。</li>
   </ul>
  </li>
  <li><code>metadata</code>
   <ul>
	<li>脚本使用的 track。 对用户不可见。</li>
   </ul>
  </li>
 </ul>
 
- label

	当列出可用的 text tracks 时，给浏览器使用的 text track 的标题，这种标题是用户可读的。
- src

	track 的地址。必须是合法的 URL。该属性必须定义。
- srclang

	track 文本数据的语言。它必须是合法的 <a href="http://people.w3.org/rishida/utils/subtags/" rel=" noopener" class="external">BCP 47</a> 语言标签。如果 <code>kind</code> 属性被设为&nbsp;<code>subtitles,</code> 那么<code>srclang</code> 必须定义。"#####
}

