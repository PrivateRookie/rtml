def! {
  thead,
  Thead,
  TheadArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<thead>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/thead)

---

**允许的内容**

零或多个<a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a>元素。

**是否可忽略关闭标签**

开头的标签是强制的。如果<a href="/zh-CN/docs/Web/HTML/Element/thead" aria-current="page"><code>&lt;thead&gt;</code></a> 元素后直接跟 <a class="only-in-en-us" href="/en-US/docs/Web/HTML/Element/tbody" title="Currently only available in English (US)"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>或<a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a>元素，结尾的标签可以被省略。

**父标签**

A <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> element. The <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a> must appear after any <a href="/zh-CN/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a> or <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> element, even implicitly defined, but before any <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>, <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> element.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)


---

## 属性

 包含全局属性: true

- align

	This enumerated attribute specifies how horizontal alignment of each cell content will be handled. Possible values are:
 <ul>
  <li><code>left</code>, aligning the content to the left of the cell</li>
  <li><code>center</code>, centering the content in the cell</li>
  <li><code>right</code>, aligning the content to the right of the cell</li>
  <li><code>justify</code>, inserting spaces into the textual content so that the content is justified in the cell</li>
  <li><code>char</code>, aligning the textual content on a special character with a minimal offset, defined by the <a href="/zh-CN/docs/Web/HTML/Element/thead#attr-char" aria-current="page"><code>char</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/thead#attr-charoff" aria-current="page"><code>charoff</code></a> attributes <span class="notecard inline warning">未实现 (查看 <a href="https://bugzilla.mozilla.org/show_bug.cgi?id=2212" class="external" rel=" noopener">bug&nbsp;2212</a>)</span>.</li>
 </ul>

 <p>If this attribute is not set,&nbsp; the <code>left</code> value is assumed.</p>

 <div id="sect1" class="note notecard"><strong>Note: </strong>Do not use this attribute as it is obsolete (not supported) in the latest standard.

 <ul>
  <li>To achieve the same effect as the <code>left</code>, <code>center</code>, <code>right</code> or <code>justify</code> values, use the CSS <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property on it.</li>
  <li>To achieve the same effect as the <code>char</code> value, in CSS3, you can use the value of the <a href="/zh-CN/docs/Web/HTML/Element/thead#attr-char" aria-current="page"><code>char</code></a> as the value of the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property <span class="notecard inline warning">未实现</span>.</li>
 </ul>
 </div>
 
- bgcolor

	This attribute defines the background color of each cell of the column. It is one of the 6-digit hexadecimal code as defined in <a rel=" noopener" href="https://www.w3.org/Graphics/Color/sRGB" class="external">sRGB</a>, prefixed by a '#'. One of the sixteen predefined color strings may be used:
 <div class="table-scroll"><table style="width: 80%;">
  <tbody>
   <tr>
	<td style="background-color: black; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>black</code> = "#000000"</td>
	<td style="background-color: green; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>green</code> = "#008000"</td>
   </tr>
   <tr>
	<td style="background-color: silver; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>silver</code> = "#C0C0C0"</td>
	<td style="background-color: lime; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>lime</code> = "#00FF00"</td>
   </tr>
   <tr>
	<td style="background-color: gray; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>gray</code> = "#808080"</td>
	<td style="background-color: olive; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>olive</code> = "#808000"</td>
   </tr>
   <tr>
	<td style="background-color: white; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>white</code> = "#FFFFFF"</td>
	<td style="background-color: yellow; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>yellow</code> = "#FFFF00"</td>
   </tr>
   <tr>
	<td style="background-color: maroon; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>maroon</code> = "#800000"</td>
	<td style="background-color: navy; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>navy</code> = "#000080"</td>
   </tr>
   <tr>
	<td style="background-color: red; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>red</code> = "#FF0000"</td>
	<td style="background-color: blue; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>blue</code> = "#0000FF"</td>
   </tr>
   <tr>
	<td style="background-color: purple; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>purple</code> = "#800080"</td>
	<td style="background-color: teal; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>teal</code> = "#008080"</td>
   </tr>
   <tr>
	<td style="background-color: fuchsia; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>fuchsia</code> = "#FF00FF"</td>
	<td style="background-color: aqua; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>aqua</code> = "#00FFFF"</td>
   </tr>
  </tbody>
 </table></div>

 <div id="sect2" class="note notecard"><strong>Usage note:&nbsp;</strong>Do not use this attribute, as it is non-standard and only implemented in some versions of Microsoft Internet Explorer: the <a href="/zh-CN/docs/Web/HTML/Element/thead" aria-current="page"><code>&lt;thead&gt;</code></a> element should be styled using <a href="/en-US/docs/Web/CSS">CSS</a>. To give a similar effect to the <strong>bgcolor</strong> attribute, use the <a href="/en-US/docs/Web/CSS">CSS</a> property <a href="/zh-CN/docs/Web/CSS/background-color"><code>background-color</code></a>, on the relevant <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> or <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a> elements.</div>
 
- char

	This attribute is used to set the character to align the cells in a column on. Typical values for this include a period (.) when attempting to align numbers or monetary values. If <a href="/zh-CN/docs/Web/HTML/Element/tr#attr-align"><code>align</code></a> is not set to <code>char</code>, this attribute is ignored.
 <div id="sect3" class="note notecard"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard. To achieve the same effect as the <a href="/zh-CN/docs/Web/HTML/Element/thead#attr-char" aria-current="page"><code>char</code></a>, in CSS3, you can use the character set using the <a href="/zh-CN/docs/Web/HTML/Element/thead#attr-char" aria-current="page"><code>char</code></a> attribute as the value of the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property <span class="notecard inline warning">未实现</span>.</div>
 
- charoff

	This attribute is used to indicate the number of characters to offset the column data from the alignment characters specified by the <strong>char</strong> attribute.
 <div id="sect4" class="note notecard"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard.</div>
 
- valign

	This attribute specifies the vertical alignment of the text within each row of cells of the table header. Possible values for this attribute are:
 <ul>
  <li><code>baseline</code>, which will put the text as close to the bottom of the cell as it is possible, but align it on the <a class="external" href="https://en.wikipedia.org/wiki/Baseline_%28typography%29" rel=" noopener">baseline</a> of the characters instead of the bottom of them. If characters are all of the size, this has the same effect as <code>bottom</code>.</li>
  <li><code>bottom</code>, which will put the text as close to the bottom of the cell as it is possible;</li>
  <li><code>middle</code>, which will center the text in the cell;</li>
  <li><code>top</code>, which will put the text as close to the top of the cell as it is possible.</li>
 </ul>

 <div class="note notecard" id="sect5"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard: instead set the CSS&nbsp;<a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> property on it.</div>
 "#####
}

