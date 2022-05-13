def! {
  col,
  Col,
  ColArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<col>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/col)

---

**允许的内容**

<dfn>Permitted content</dfn> None, it is an <a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>.

**是否可忽略关闭标签**

<dfn>Tag omission</dfn> The start tag is mandatory, but, as it is a void element, the use of an end tag is forbidden.

**父标签**

<dfn>Permitted parent elements</dfn> <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> only, though it can be implicitly defined as its start tag is not mandatory. The <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> must not have a <a href="/zh-CN/docs/Web/HTML/Element/colgroup#attr-span"><code>span</code></a> attribute.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)


---

## 属性

 包含全局属性: true

- align

	This enumerated attribute specifies how horizontal alignment of each column cell content will be handled. Possible values are:
 <ul>
  <li><code>left</code>, aligning the content to the left of the cell</li>
  <li><code>center</code>, centering the content in the cell</li>
  <li><code>right</code>, aligning the content to the right of the cell</li>
  <li><code>justify</code>, inserting spaces into the textual content so that the content is justified in the cell</li>
  <li><code>char</code>, aligning the textual content on a special character with a minimal offset, defined by the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/col#attr-char"><code>char</code></a> and <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/col#attr-charoff"><code>charoff</code></a> attributes <span class="notecard inline warning">未实现 (查看 <a class="external" href="https://bugzilla.mozilla.org/show_bug.cgi?id=2212" rel=" noopener">bug&nbsp;2212</a>)</span>.</li>
 </ul>

 <p>If this attribute is not set, its value is inherited from the <a href="/zh-CN/docs/Web/HTML/Element/colgroup#attr-align"><code>align</code></a> of the <a href="/zh-CN/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a> element this <code>&lt;col&gt;</code> element belongs too. If there are none, the <code>left</code> value is assumed.</p>

 <div class="note notecard" id="sect1"><strong>Note: </strong>Do not use this attribute as it is obsolete (not supported) in the latest standard.

 <ul>
  <li>To achieve the same effect as the <code>left</code>, <code>center</code>, <code>right</code> or <code>justify</code> values:

   <ul>
	<li>Do not try to set the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property on a selector giving a <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/col"><code>&lt;col&gt;</code></a> element. Because <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> elements are not descendant of the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/col"><code>&lt;col&gt;</code></a> element, they won't inherit it.</li>
	<li>If the table doesn't use a <a href="/zh-CN/docs/Web/HTML/Element/td#attr-colspan"><code>colspan</code></a> attribute, use the <code>td:nth-child(an+b)</code> CSS selector where a is the total number of the columns in the table and b is the ordinal position of the column in the table. Only after this selector the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property can be used.</li>
	<li>If the table does use a <a href="/zh-CN/docs/Web/HTML/Element/td#attr-colspan"><code>colspan</code></a> attribute, the effect can be achieved by combining adequate CSS attribute selectors like <code>[colspan=n]</code>, though this is not trivial.</li>
   </ul>
  </li>
  <li>To achieve the same effect as the <code>char</code> value, in CSS3, you can use the value of the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/col#attr-char"><code>char</code></a> as the value of the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property <span class="notecard inline warning">未实现</span>.</li>
 </ul>
 </div>
 
- bgcolor

	定义列中对应的每个单元格的背景色。其值是 <a href="https://www.w3.org/Graphics/Color/sRGB" class="external" rel=" noopener">sRGB</a> 定义的 6 位 16 进制代码之一，前缀为 '#'。 也可以使用以下 16 个预定义的颜色字符：
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

 <div class="note notecard" id="sect2"><strong>注意：不要使用这个属性</strong>, 它是非标准的，并且只实现了 IE 浏览器中的部分版本: <a href="/zh-CN/docs/Web/HTML/Element/col" aria-current="page"><code>&lt;col&gt;</code></a> 元素的样式应该使用 <a href="/en-US/docs/Web/CSS" title="CSS">CSS</a>. 在<a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a>元素上使用 <a href="/en-US/docs/Web/CSS" title="CSS">CSS</a> 的 <a href="/zh-CN/docs/Web/CSS/background-color"><code>background-color</code></a>&nbsp;属性即可实现相同效果。</div>
 
- char

	This attribute is used to set the character to align the cells in a column on. Typical values for this include a period (.) when attempting to align numbers or monetary values. If <a href="/zh-CN/docs/Web/HTML/Element/col#attr-align" aria-current="page"><code>align</code></a> is not set to <code>char</code>, this attribute is ignored.
 <div class="note notecard" id="sect3"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard. To achieve the same effect as the <a href="/zh-CN/docs/Web/HTML/Element/col#attr-char" aria-current="page"><code>char</code></a>, in CSS3, you can use the character set using the <a href="/zh-CN/docs/Web/HTML/Element/col#attr-char" aria-current="page"><code>char</code></a> attribute as the value of the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property <span class="notecard inline warning">未实现</span>.</div>
 
- charoff

	This attribute is used to indicate the number of characters to offset the column data from the alignment characters specified by the <strong>char</strong> attribute.
 <div id="sect4" class="note notecard"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard.</div>
 
- span

	该属性值为一个正整数，表示该 &lt;col&gt; 元素横跨的列数。默认值为 1
- valign

	This attribute specifies the vertical alignment of the text within each cell of the column. Possible values for this attribute are:
 <ul>
  <li><code>baseline</code>, which will put the text as close to the bottom of the cell as it is possible, but align it on the <a rel=" noopener" href="http://en.wikipedia.org/wiki/Baseline_%28typography%29" class="external">baseline</a> of the characters instead of the bottom of them. If characters are all of the size, this has the same effect as <code>bottom</code>.</li>
  <li><code>bottom</code>, which will put the text as close to the bottom of the cell as it is possible;</li>
  <li><code>middle</code>, which will center the text in the cell;</li>
  <li>and <code>top</code>, which will put the text as close to the top of the cell as it is possible.</li>
 </ul>

 <div class="note notecard" id="sect5"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard:

 <ul>
  <li>Do not try to set the <a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> property on a selector giving a <a href="/zh-CN/docs/Web/HTML/Element/col" aria-current="page"><code>&lt;col&gt;</code></a> element. Because <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> elements are not descendant of the <a href="/zh-CN/docs/Web/HTML/Element/col" aria-current="page"><code>&lt;col&gt;</code></a> element, they won't inherit it.</li>
  <li>If the table doesn't use a <a href="/zh-CN/docs/Web/HTML/Element/td#attr-colspan"><code>colspan</code></a> attribute, use the <code>td:nth-child(an+b)</code> CSS selector where a is the total number of the columns in the table and b is the ordinal position of the column in the table. Only after this selector the <a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> property can be used.</li>
  <li>If the table does use a <a href="/zh-CN/docs/Web/HTML/Element/td#attr-colspan"><code>colspan</code></a> attribute, the effect can be achieved by combining adequate CSS attribute selectors like <code>[colspan=n]</code>, though this is not trivial.</li>
 </ul>
 </div>
 
- width

	This attribute specifies a default width for each column in the current column group. In addition to the standard pixel and percentage values, this attribute might take the special form <code>0*</code>, which means that the width of each column in the group should be the minimum width necessary to hold the column's contents. Relative widths such as <code>0.5*</code> also can be used."#####
}

