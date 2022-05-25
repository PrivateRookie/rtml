def! {
  tfoot,
  Tfoot,
  TfootArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<tfoot>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/tfoot)

---

**允许的内容**

0或多个<a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素。

**是否可忽略关闭标签**

开始标签是必需的。.在父元素 <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> 没有后续内容的情况下，结束标签可被省略。

**父标签**

<a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> 元素。<a href="/zh-CN/docs/Web/HTML/Element/tfoot" aria-current="page"><code>&lt;tfoot&gt;</code></a> 必须出现在一个或多个 <a href="/zh-CN/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a>，<a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a>，<a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>，或 <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素之后。 注意这是自 HTML5 起有的要求。<br>
	<span class="badge inline html-version"><a href="/zh-CN/docs/Web/HTML">HTML 4</a></span> <a href="/zh-CN/docs/Web/HTML/Element/tfoot" aria-current="page"><code>&lt;tfoot&gt;</code></a> 元素不能放在任何 <a href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us" title="Currently only available in English (US)"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a> 或 <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素之后。注意，这与上述 HTML5 的标准相冲突。

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)


---

## 属性

 包含全局属性: true

- align

	
 <p>此枚举属性指定每个单元格内容所使用的水平对齐方式。可选值为：</p>

 <ul>
  <li><code>left</code>，单元格内容左对齐</li>
  <li><code>center</code>，单元格内容居中对齐</li>
  <li><code>right</code>，单元格内容右对齐</li>
  <li><code>justify</code>，插入空白调整单元格中的文本内容（译者注：即两端对齐）</li>
  <li><code>char</code>，将文本内容与一个具有最小偏移量的特定字符对齐，字符和偏移量分别由<a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody"><code>char</code> <small>(en-US)</small></a>和<a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us"><code>charoff</code> <small>(en-US)</small></a>属性定义。<span class="notecard inline warning">未实现 (查看 <a href="https://bugzilla.mozilla.org/show_bug.cgi?id=2212" rel=" noopener" class="external">bug&nbsp;2212</a>)</span>.</li>
 </ul>

 <p>若此值未设置，则假定为<code>left</code>。</p>

 <div id="sect1" class="note notecard">
 <p><strong>Note: </strong>此属性在最新标准中已被废弃（不支持），所以请勿使用。</p>

 <ul>
  <li>为达到与<code>left</code>, <code>center</code>, <code>right</code>或<code>justify</code>相同的效果，请使用 CSS <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a>属性。</li>
  <li>为达到与 char 值相同的效果，在 CSS3 中可将<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/tfoot#attr-char"><code>char</code></a>的值用作<a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a>的属性值<span class="notecard inline warning">未实现</span>。</li>
 </ul>
 </div>
 
- bgcolor

	此属性定义了列内单元格的背景色。定义此属性使用'#'作为前缀，其后是定义于<a class="external" href="https://www.w3.org/Graphics/Color/sRGB" rel=" noopener">sRGB</a>的 6 位十六进制码。
 <p>也可使用以下 16 种预定义的色彩字符串之一：</p>

 <div class="table-scroll"><table style="width: 80%;">
  <tbody>
   <tr>
	<td style="background-color: black; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>black</code> = "#000000"</td>
	<td style="background-color: green; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>green</code> = "#008000"</td>
   </tr>
   <tr>
	<td style="background-color: silver; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>silver</code> = "#C0C0C0"</td>
	<td style="background-color: lime; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>lime</code> = "#00FF00"</td>
   </tr>
   <tr>
	<td style="background-color: gray; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>gray</code> = "#808080"</td>
	<td style="background-color: olive; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>olive</code> = "#808000"</td>
   </tr>
   <tr>
	<td style="background-color: white; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>white</code> = "#FFFFFF"</td>
	<td style="background-color: yellow; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>yellow</code> = "#FFFF00"</td>
   </tr>
   <tr>
	<td style="background-color: maroon; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>maroon</code> = "#800000"</td>
	<td style="background-color: navy; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>navy</code> = "#000080"</td>
   </tr>
   <tr>
	<td style="background-color: red; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>red</code> = "#FF0000"</td>
	<td style="background-color: blue; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>blue</code> = "#0000FF"</td>
   </tr>
   <tr>
	<td style="background-color: purple; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>purple</code> = "#800080"</td>
	<td style="background-color: teal; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>teal</code> = "#008080"</td>
   </tr>
   <tr>
	<td style="background-color: fuchsia; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>fuchsia</code> = "#FF00FF"</td>
	<td style="background-color: aqua; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;">&nbsp;</td>
	<td><code>aqua</code> = "#00FFFF"</td>
   </tr>
  </tbody>
 </table></div>

 <div id="sect2" class="note notecard"><strong>Usage Note: </strong>请勿使用此属性，因为这并非标准，且只有某些特定版本的 Microsoft Internet Explorer（IE 浏览器）支持：<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a>元素应使用<a href="/en-US/docs/Web/CSS">CSS</a>设计。若想得到与<strong>bgcolor</strong>属性相似的效果，可在相关的 <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a>或<a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a>元素中使用<a href="/en-US/docs/Web/CSS">CSS</a> <a href="/zh-CN/docs/Web/CSS/background-color"><code>background-color</code></a>属性。</div>
 
- char

	此属性设置单元格对齐的基准字符。当对齐数字或货币值时，一个典型值会带有一个句点 (.)。如果<a href="/zh-CN/docs/Web/HTML/Element/tfoot#attr-align" aria-current="page"><code>align</code></a>未设置为<code>char</code>，此属性将被忽略。
 <div class="note notecard" id="sect3"><strong>Note: </strong>请勿使用此属性，因为在最新标准中此属性被废弃（且不受支持）。想要达到与<a class="page-not-created" title="此页面仍未被本地化, 期待您的翻译!"><code>char</code></a>相同的效果，在 CSS3 中，可将<a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a>属性设置为<a href="/zh-CN/docs/Web/HTML/Element/tfoot#attr-char" aria-current="page"><code>char</code></a>的属性值<span class="notecard inline warning">未实现</span>。</div>
 
- charoff

	此属性用作表明列内数据对于对齐基准字符的偏移字符数，对其基准字符由<code>char</code>属性指定。
 <div class="note notecard" id="sect4"><strong>Note: </strong> 请勿使用此属性，因为在最新标准中此属性被废弃（且不受支持）。</div>
 
- valign

	此属性指定表头（译者注：英文原文为 table header，疑似错误）中每一行内的文本的垂直对齐方式。此属性的可选值为：
 <ul>
  <li><code>baseline</code>，会使文本尽可能接近单元格底部，但并不向底部对齐，而将它们以字符<a rel=" noopener" href="https://zh.wikipedia.org/wiki/%E5%9F%BA%E7%B7%9A" class="external">基线</a>对齐。如果所有字符的大小相同，此值与<code>bottom</code>的效果相同；</li>
  <li><code>bottom</code>，设置文本向下对齐；</li>
  <li><code>middle</code>，设置文本居中对齐；</li>
  <li>and <code>top</code>，设置文本向上对齐。</li>
 </ul>

 <div class="note notecard" id="sect5"><strong>Note: </strong> 请勿使用此属性，因为在最新标准中此属性被废弃（且不受支持）：使用 CSS 属性<a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a>作为替代。</div>
 "#####
}
