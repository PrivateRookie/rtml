def! {
  td,
  Td,
  TdArg,
  doc:
  "zh-CN" = r#####"`<td>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/td)

---

**允许的内容**

<a href="/en-US/docs/Web/Guide/HTML/Content_categories#flow_content">Flow content</a>.

**是否可忽略关闭标签**

The start tag is mandatory.<br>
	The end tag may be omitted, if it is immediately followed by a <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a> or <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> element or if there are no more data in its parent element.

**父标签**

<a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素。

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)


---

## 属性

 包含全局属性: true

- abbr

	This attribute contains a short abbreviated description of the cell's content. Some user-agents, such as speech readers, may present this description before the content itself.
 <div class="note notecard" id="sect3"><strong>Note:</strong> Do not use this attribute as it is obsolete in the latest standard. Alternatively, you can put the abbreviated description inside the cell and place the long content in the <strong>title</strong> attribute.</div>
 
- align

	This enumerated attribute specifies how the cell content's horizontal alignment will be handled. Possible values are:
 <ul>
  <li><code>left</code>: The content is aligned to the left of the cell.</li>
  <li><code>center</code>: The content is centered in the cell.</li>
  <li><code>right</code>: The content is aligned to the right of the cell.</li>
  <li><code>justify</code> (with text only): The content is stretched out inside the cell so that it covers its entire width.</li>
  <li><code>char</code> (with text only): The content is aligned to a character inside the <code>&lt;th&gt;</code> element with minimal offset. This character is defined by the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/td#attr-char"><code>char</code></a> and <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/td#attr-charoff"><code>charoff</code></a> attributes <span class="notecard inline warning">未实现 (查看 <a class="external" href="https://bugzilla.mozilla.org/show_bug.cgi?id=2212" rel=" noopener">bug&nbsp;2212</a>)</span>.</li>
 </ul>

 <p>The default value when this attribute is not specified is <code>left</code>.</p>

 <div class="note notecard" id="sect4"><strong>Note: </strong>Do not use this attribute as it is obsolete in the latest standard.

 <ul>
  <li>To achieve the same effect as the <code>left</code>, <code>center</code>, <code>right</code> or <code>justify</code> values, apply the CSS <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property to the element.</li>
  <li>To achieve the same effect as the <code>char</code> value, give the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property the same value you would use for the <a href="/zh-CN/docs/Web/HTML/Element/td#attr-char" aria-current="page"><code>char</code></a>. <span class="notecard inline warning">未实现</span> in CSS3.</li>
 </ul>
 </div>
 
- axis

	This attribute contains a list of space-separated strings. Each string is the <code>id</code> of a group of cells that this header applies to.
 <div class="note notecard" id="sect5"><strong>Note:</strong> Do not use this attribute as it is obsolete in the latest standard.</div>
 
- bgcolor

	This attribute defines the background color of each cell in a column. It consists of a 6-digit hexadecimal code as defined in <a href="https://www.w3.org/Graphics/Color/sRGB" class="external" rel=" noopener">sRGB</a> and is prefixed by '#'. This attribute may be used with one of sixteen predefined color strings:
 <div class="table-scroll"><table>
  <tbody>
   <tr>
	<td style="background-color: black; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>black</code> = "#000000"</td>
	<td style="background-color: green; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>green</code> = "#008000"</td>
   </tr>
   <tr>
	<td style="background-color: silver; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>silver</code> = "#C0C0C0"</td>
	<td style="background-color: lime; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>lime</code> = "#00FF00"</td>
   </tr>
   <tr>
	<td style="background-color: gray; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>gray</code> = "#808080"</td>
	<td style="background-color: olive; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>olive</code> = "#808000"</td>
   </tr>
   <tr>
	<td style="background-color: white; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>white</code> = "#FFFFFF"</td>
	<td style="background-color: yellow; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>yellow</code> = "#FFFF00"</td>
   </tr>
   <tr>
	<td style="background-color: maroon; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>maroon</code> = "#800000"</td>
	<td style="background-color: navy; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>navy</code> = "#000080"</td>
   </tr>
   <tr>
	<td style="background-color: red; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>red</code> = "#FF0000"</td>
	<td style="background-color: blue; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>blue</code> = "#0000FF"</td>
   </tr>
   <tr>
	<td style="background-color: purple; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>purple</code> = "#800080"</td>
	<td style="background-color: teal; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>teal</code> = "#008080"</td>
   </tr>
   <tr>
	<td style="background-color: fuchsia; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>fuchsia</code> = "#FF00FF"</td>
	<td style="background-color: aqua; width: 24px; height: 24px; border: 1px solid black;"></td>
	<td><code>aqua</code> = "#00FFFF"</td>
   </tr>
  </tbody>
 </table></div>

 <div class="note notecard" id="sect6"><strong>Note:</strong> Do not use this attribute, as it is non-standard and only implemented in some versions of Microsoft Internet Explorer: The <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> element should be styled using <a href="/en-US/docs/Web/CSS">CSS</a>. To create a similar effect use the <a href="/zh-CN/docs/Web/CSS/background-color"><code>background-color</code></a> property in <a href="/en-US/docs/Web/CSS">CSS</a> instead.</div>
 
- char

	The content in the cell element is aligned to a character. Typical values include a period (.) to align numbers or monetary values. If <a href="/zh-CN/docs/Web/HTML/Element/td#attr-align" aria-current="page"><code>align</code></a> is not set to <code>char</code>, this attribute is ignored.
 <div class="note notecard" id="sect7"><strong>Note:</strong> Do not use this attribute as it is obsolete in the latest standard. To achieve the same effect, you can specify the character as the first value of the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property, <span class="notecard inline warning">未实现</span> in CSS3.</div>
 
- charoff

	This attribute is used to shift column data to the right of the character specified by the <strong>char</strong> attribute. Its value specifies the length of this shift.
 <div id="sect8" class="note notecard"><strong>Note: </strong>Do not use this attribute as it is obsolete in the latest standard.</div>
 
- colspan

	This attribute contains a non-negative integer value that indicates for how many columns the cell extends. Its default value is <code>1</code>. Values higher than 1000 will be considered as incorrect and will be set to the default value (1).
- headers

	This attribute contains a list of space-separated strings, each corresponding to the <strong>id</strong> attribute of the <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a> elements that apply to this element.
- height

	This attribute is used to define a recommended cell height.
 <div id="sect9" class="note notecard"><strong>Note:</strong> Do not use this attribute as it is obsolete in the latest standard: use the CSS <a href="/zh-CN/docs/Web/CSS/height"><code>height</code></a> property instead.</div>
 
- rowspan

	This attribute contains a non-negative integer value that indicates for how many rows the cell extends. Its default value is <code>1</code>; if its value is set to <code>0</code>, it extends until the end of the table section (<a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us" title="Currently only available in English (US)"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>, <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a>, even if implicitly defined), that the cell belongs to. Values higher than 65534 are clipped down to 65534.
- scope

	This enumerated attribute defines the cells that the header (defined in the <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a>) element relates to.
 <div id="sect10" class="note notecard"><strong>Note:</strong> Using this attribute with a table cell element is obsolete in the latest standard. Use this attribute only with the <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a> element to define the row or column for which it is a header.</div>
 
- valign

	This attribute specifies how a text is vertically aligned inside a cell. Possible values for this attribute are:
 <ul>
  <li><code>baseline</code>: Positions the text near the bottom of the cell and aligns it with the <a rel=" noopener" class="external" href="https://en.wikipedia.org/wiki/Baseline_%28typography%29">baseline</a> of the characters instead of the bottom. If characters don't descend below the baseline, the baseline value achieves the same effect as <code>bottom</code>.</li>
  <li><code>bottom</code>: Positions the text near the bottom of the cell.</li>
  <li><code>middle</code>: Centers the text in the cell.</li>
  <li>and <code>top</code>: Positions the text near the top of the cell.</li>
 </ul>

 <div id="sect11" class="note notecard"><strong>Note:</strong> Do not use this attribute as it is obsolete in the latest standard: use the CSS <a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> property instead.</div>
 
- width

	This attribute is used to define a recommended cell width. Additional space can be added with the <a href="/en-US/docs/Web/API/HTMLTableElement/cellSpacing">cellspacing</a> and <a href="/en-US/docs/Web/API/HTMLTableElement/cellPadding">cellpadding</a> properties and the width of the <a href="/zh-CN/docs/Web/HTML/Element/col"><code>&lt;col&gt;</code></a> element can also create extra width. But, if a column's width is too narrow to show a particular cell properly, it will be widened when displayed.
 <div id="sect12" class="note notecard"><strong>Note:</strong> Do not use this attribute as it is obsolete in the latest standard: use the CSS <a href="/zh-CN/docs/Web/CSS/width"><code>width</code></a> property instead.</div>
 "#####
}

