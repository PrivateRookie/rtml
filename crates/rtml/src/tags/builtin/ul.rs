def_tag! {
  ul,
  Ul,
  UlArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<ul>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/ul)

---

**允许的内容**

<a href="/en-US/HTML/Content_categories#flow_content" title="en/HTML/Content categories#Flow content">流式内容</a>，如果 <code>&lt;ul&gt;</code> 包含至少一个 <code>&lt;li&gt;</code> 元素，那么它就是显性内容 <a href="/en-US/docs/Web/Guide/HTML/Content_categories#Palpable_content">Palpable content</a>。

**是否可忽略关闭标签**

允许的内容

**父标签**

零个或更多个 <a href="/zh-CN/docs/Web/HTML/Element/li"><code>&lt;li&gt;</code></a> 元素，可以混合使用 <a href="/zh-CN/docs/Web/HTML/Element/ol"><code>&lt;ol&gt;</code></a> 与<a href="/zh-CN/docs/Web/HTML/Element/ul" aria-current="page"><code>&lt;ul&gt;</code></a> 元素。

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement)


---

## 属性

 包含全局属性: true

- compact

	此布尔属性提示列表是否需要被渲染为更紧凑的样式。用户代理决定如何解释这个属性，且并非所有浏览器都支持它。
 <div class="note notecard" id="sect2"><strong>使用说明：</strong>不要使用这个属性，因为它已经被废弃了：<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/ul"><code>&lt;ul&gt;</code></a> 元素应当使用&nbsp;<a href="/zh-CN/docs/Web/CSS">CSS</a> 来更改样式。（CSS）可以提供与<code> compact</code> 属性相同的效果，将&nbsp; <a href="/zh-CN/docs/Web/CSS">CSS</a> 属性 <a title="Currently only available in English (US)" href="/en-US/docs/Web/CSS/line-height" class="only-in-en-us">line-height (en-US)</a> 的值设为 <code>80%</code> 即可。</div>
 
- type

	用于设置列表的着重号样式&nbsp;，被定义在&nbsp;<a href="/en-US/HTML3.2" title="en/HTML3.2">HTML3.2</a> 和过渡版本 &nbsp;<a href="/en-US/HTML4.01" title="en/HTML4.01">HTML 4.0/4.01</a> 中的可用值有:
 <ul>
  <li><code>circle</code></li>
  <li><code>disc</code></li>
  <li><code>square</code></li>
 </ul>

 <p>第四种着重号样式被定义在 WebTV 接口中，但并不是所有浏览器都支持：<code>triangle</code></p>

 <p>如果未设置此 HTML 属性且没有&nbsp;<a title="en/CSS" href="/en-US/CSS">CSS</a> <a href="/zh-CN/docs/Web/CSS/list-style-type"><code>list-style-type</code></a> 属性作用于这个元素，用户代理会决定使用哪种着重号样式，一般来说这也和嵌套的层级数有关。</p>

 <div id="sect3" class="note notecard"><strong>使用说明：</strong> 不要使用这个属性，它已经被废弃了：使用 <a href="/en-US/CSS" title="en/CSS">CSS</a> <a href="/zh-CN/docs/Web/CSS/list-style-type"><code>list-style-type</code></a> 属性作为代替。</div>
 "#####
}
