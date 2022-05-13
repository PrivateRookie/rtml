def! {
  tr,
  Tr,
  TrArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<tr>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/tr)

---

**允许的内容**

Zero or more <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> or <a href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a> elements, or a mix of them

**是否可忽略关闭标签**

Start tag is mandatory. End tag may be omitted if the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> element is immediately followed by a <a href="/zh-CN/docs/Web/HTML/Element/tr" aria-current="page"><code>&lt;tr&gt;</code></a> element, or if the parent table group (<a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a> or <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a>) element doesn't have any more content

**父标签**

<a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a>, <a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, <a title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody" class="only-in-en-us"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a> or <a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> element

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLTableRowElement)


---

## 属性

 包含全局属性: true

- align

	该枚举属性指定每个单元格中内容的水平对齐方式，可使用的属性值如下：
 <ul>
  <li><code>left</code>,&nbsp;内容在单元格中左对齐</li>
  <li><code>center</code>,&nbsp;内容在单元格中居中</li>
  <li><code>right</code>,&nbsp;内容在单元格中右对齐</li>
  <li><code>justify</code>,&nbsp;增加文本内容之间的空白以伸展这行文本，使得该单元格中的多行文本具有相同的长度。</li>
  <li><code>char</code>, aligning the textual content on a special character with a minimal offset, defined by the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/tr#attr-char"><code>char</code></a> and <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/tr#attr-charoff"><code>charoff</code></a> attributes <span class="notecard inline warning">未实现 (查看 <a class="external" rel=" noopener" href="https://bugzilla.mozilla.org/show_bug.cgi?id=2212">bug&nbsp;2212</a>)</span></li>
 </ul>

 <p>If this attribute is not set, the parent node's value is inherited.</p>

 <div class="note notecard" id="sect1"><strong>Note: </strong>Do not use this attribute as it is obsolete (not supported) in the latest standard.

 <ul>
  <li>To achieve the same effect as the <code>left</code>, <code>center</code>, <code>right</code> or <code>justify</code> values, use the CSS <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property on it.</li>
  <li>To achieve the same effect as the <code>char</code> value, in CSS3, you can use the value of the <a href="/zh-CN/docs/Web/HTML/Element/tr#attr-char" aria-current="page"><code>char</code></a> as the value of the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property <span class="notecard inline warning">未实现</span>.</li>
 </ul>
 </div>
 
- bgcolor

	This attribute defines the background color of each cell of the row. It can be either an <a href="/en-US/docs/Web/CSS/color_value#rgb()">hexadecimal <code>#RRGGBB</code> or <code>#RGB</code> value</a> or a <a href="/en-US/docs/Web/CSS/color_value#color_keywords">color keyword</a>.
 <div class="note notecard" id="sect2"><strong>Usage note: </strong>the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> element should be styled using <a href="/en-US/docs/Web/CSS">CSS</a>. To give a similar effect to the <strong>bgcolor</strong> attribute, use the <a href="/en-US/docs/Web/CSS">CSS</a> property <a href="/zh-CN/docs/Web/CSS/background-color"><code>background-color</code></a>.</div>
 
- char

	This attribute is used to set the character to align the cells in a column on. Typical values for this include a period (.) when attempting to align numbers or monetary values. If <a href="/zh-CN/docs/Web/HTML/Element/tr#attr-align" aria-current="page"><code>align</code></a> is not set to <code>char</code>, this attribute is ignored.
 <div class="note notecard" id="sect3"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard. To achieve the same effect as the <a href="/zh-CN/docs/Web/HTML/Element/tr#attr-char" aria-current="page"><code>char</code></a>, in CSS3, you can use the character set using the <a href="/zh-CN/docs/Web/HTML/Element/tr#attr-char" aria-current="page"><code>char</code></a> attribute as the value of the <a href="/zh-CN/docs/Web/CSS/text-align"><code>text-align</code></a> property <span class="notecard inline warning">未实现</span>.</div>
 
- charoff

	This attribute is used to indicate the number of characters to offset the column data from the alignment characters specified by the <strong>char</strong> attribute.
 <div class="note notecard" id="sect4"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard.</div>
 
- valign

	This attribute specifies the vertical alignment of the text within each row of cells of the table header. Possible values for this attribute are:
 <ul>
  <li><code>baseline</code>, which will put the text as close to the bottom of the cell as it is possible, but align it on the <a class="external" href="https://en.wikipedia.org/wiki/Baseline_%28typography%29" rel=" noopener">baseline</a> of the characters instead of the bottom of them. If characters are all of the size, this has the same effect as <code>bottom</code>.</li>
  <li><code>bottom</code>, which will put the text as close to the bottom of the cell as it is possible;</li>
  <li><code>middle</code>, which will center the text in the cell;</li>
  <li>and <code>top</code>, which will put the text as close to the top of the cell as it is possible.</li>
 </ul>

 <div class="note notecard" id="sect5"><strong>Note: </strong>Do not use this attribute as it is obsolete (and not supported) in the latest standard: instead set the CSS&nbsp;<a href="/zh-CN/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> property on it.</div>
 "#####
}

