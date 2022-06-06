def_tag! {
  colgroup,
  Colgroup,
  ColgroupArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<colgroup>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/colgroup)

---

**允许的内容**

If the <a href="/zh-CN/docs/Web/HTML/Element/colgroup#attr-span" aria-current="page"><code>span</code></a> attribute is present: none, it is an <a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>.<br>
	If the attribute is not present: zero or more <a href="/zh-CN/docs/Web/HTML/Element/col"><code>&lt;col&gt;</code></a> element

**是否可忽略关闭标签**

如果元素的第一个子元素存在且是一个 <a href="/zh-CN/docs/Web/HTML/Element/col"><code>&lt;col&gt;</code></a> 元素，而且在它之前没有省略了结束标签的 <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> 元素，元素的开始标签可以被省略。<br>
	如果之后没有紧跟一个空格或注释，元素的结束标签可以被省略。

**父标签**

一个 <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> 元素。The <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> must appear after any optional <a href="/zh-CN/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a> element but before any <a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a>, <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a>, <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> element.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)


---

## 属性

 包含全局属性: true

- align

	这个可枚举的属性定义每一列的元素内容的水平对齐方式，支持的值有：
- bgcolor

	这个属性用于定义列组中的每一个列成员的背景颜色。在 <a class="external" rel=" noopener" href="https://www.w3.org/Graphics/Color/sRGB">sRGB</a> 的定义中，它是一个以 '#' 开头的 6 位 16 进制值，有 16 个预定义的表示颜色的字符串可以使用，如下所示：
 <div class="table-scroll"><table>
  <tbody>
   <tr>
	<td style="background-color: black; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp; black = "#000000"</code></td>
	<td style="background-color: green; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp;green = "#008000"</code></td>
   </tr>
   <tr>
	<td style="background-color: silver; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp;silver = "#C0C0C0"</code></td>
	<td style="background-color: lime; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp; lime = "#00FF00"</code></td>
   </tr>
   <tr>
	<td style="background-color: gray; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp;&nbsp; gray = "#808080"</code></td>
	<td style="background-color: olive; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp;olive = "#808000"</code></td>
   </tr>
   <tr>
	<td style="background-color: white; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp; white = "#FFFFFF"</code></td>
	<td style="background-color: yellow; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>yellow = "#FFFF00"</code></td>
   </tr>
   <tr>
	<td style="background-color: maroon; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp;maroon = "#800000"</code></td>
	<td style="background-color: navy; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp; navy = "#000080"</code></td>
   </tr>
   <tr>
	<td style="background-color: red; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp; &nbsp; red = "#FF0000"</code></td>
	<td style="background-color: blue; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp; blue = "#0000FF"</code></td>
   </tr>
   <tr>
	<td style="background-color: purple; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp;purple = "#800080"</code></td>
	<td style="background-color: teal; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp; teal = "#008080"</code></td>
   </tr>
   <tr>
	<td style="background-color: fuchsia; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>fuchsia = "#FF00FF"</code></td>
	<td style="background-color: aqua; width: 24px; height: 24px; border-width: 1px; border-color: black; border-style: solid;"></td>
	<td><code>&nbsp; aqua = "#00FFFF"</code></td>
   </tr>
  </tbody>
 </table></div>

 <div class="note notecard" id="sect4"><strong>小贴士：</strong><strong> </strong>不要使用这个并未标准化的属性，它只在 IE 的某些版本中生效，<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> 标签应该使用 CSS 来定义样式。要实现和 <strong>bgcolor</strong> 属性相似的效果，可以在相关的 <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> 标签上使用 <a href="/zh-CN/docs/Web/CSS/background-color"><code>background-color</code></a> 属性。</div>
 
- char

	This attribute specifies the alignment of the content in a column group to a character. Typical values for this include a period (.) when attempting to align numbers or monetary values. If <a href="/zh-CN/docs/Web/HTML/Element/colgroup#attr-align" aria-current="page"><code>align</code></a> is not set to <code>char</code>, this attribute is ignored, though it will still be used as the default value for the <a href="/zh-CN/docs/Web/HTML/Element/col#attr-align"><code>align</code></a> of the <a href="/zh-CN/docs/Web/HTML/Element/col"><code>&lt;col&gt;</code></a> which are members of this column group.
 <div class="note notecard" id="sect5"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard. To achieve the same effect as the <a href="/zh-CN/docs/Web/HTML/Element/colgroup#attr-char" aria-current="page"><code>char</code></a>, in CSS3, you can use the character set using the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/colgroup#attr-char"><code>char</code></a> attribute as the value of the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property <span class="notecard inline warning">未实现</span>.</div>
 
- charoff

	This attribute is used to indicate the number of characters to offset the column data from the alignment character specified by the <strong>char</strong> attribute.
 <div class="note notecard" id="sect6"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard.</div>
 
- span

	This attribute contains a positive integer indicating the number of consecutive columns the <code>&lt;colgroup&gt;</code> element spans. If not present, its default value is <code>1</code>.
 <div id="sect7" class="note notecard"><strong>Note: </strong>This attribute is applied on the attributes of the column group, it has no effect on the CSS styling rules associated with it or, even more, to the cells of the column's members of the group.
 <ul>
  <li>The <code>span</code> attribute is not permitted if there are one or more <code>&lt;col&gt;</code> elements within the <code>&lt;colgroup&gt;</code>.</li>
 </ul>
 </div>
 
- valign

	This attribute specifies the vertical alignment of the text within each cell of the column. Possible values for this attribute are:
 <ul>
  <li><code>baseline</code>, which will put the text as close to the bottom of the cell as it is possible, but align it on the <a href="https://en.wikipedia.org/wiki/Baseline_%28typography%29" class="external" rel=" noopener">baseline</a> of the characters instead of the bottom of them. If characters are all of the size, this has the same effect as <code>bottom</code>.</li>
  <li><code>bottom</code>, which will put the text as close to the bottom of the cell as it is possible;</li>
  <li><code>middle</code>, which will center the text in the cell;</li>
  <li>and <code>top</code>, which will put the text as close to the top of the cell as it is possible.</li>
 </ul>

 <div class="note notecard" id="sect8"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard:

 <ul>
  <li>Do not try to set the <a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> property on a selector giving a <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> element. Because <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> elements are not descendant of the <a href="/zh-CN/docs/Web/HTML/Element/colgroup" aria-current="page"><code>&lt;colgroup&gt;</code></a> element, they won't inherit it.</li>
  <li>If the table doesn't use a <a href="/zh-CN/docs/Web/HTML/Element/td#attr-colspan"><code>colspan</code></a> attribute, use the <code>td:nth-child(an+b)</code> CSS selector per column, where a is the total number of the columns in the table and b is the ordinal position of the column in the table. Only after this selector the <a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> property can be used.</li>
  <li>If the table does use a <a href="/zh-CN/docs/Web/HTML/Element/td#attr-colspan"><code>colspan</code></a> attribute, the effect can be achieved by combining adequate CSS attribute selectors like <code>[colspan=n]</code>, though this is not trivial.</li>
 </ul>
 </div>
 
- width

	This attribute specifies a default width for each column in the current column group. In addition to the standard pixel and percentage values, this attribute might take the special form <code>0*</code>, which means that the width of each column in the group should be the minimum width necessary to hold the column's contents. Relative widths such as <code>0.5*</code> also can be used."#####
}
