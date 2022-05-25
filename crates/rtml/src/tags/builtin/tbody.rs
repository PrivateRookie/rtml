def! {
  tbody,
  Tbody,
  TbodyArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<tbody>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/tbody)

---

**允许的内容**

Zero or more <a href="/en-US/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> elements.

**是否可忽略关闭标签**

The <code>&lt;tbody&gt;</code> element is not a required child element for a parent <a href="/en-US/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> element to graphically render. However, it must be present, if the parent <a href="/en-US/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> element has a <a href="/en-US/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>, a <a href="/en-US/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a> or another <a href="/en-US/docs/Web/HTML/Element/tbody" aria-current="page"><code>&lt;tbody&gt;</code></a> element as a child. If the <code>&lt;tbody&gt;</code> element starts with a <a href="/en-US/docs/Web/HTML/Element/tbody" aria-current="page"><code>&lt;tbody&gt;</code></a> element, and is not following a non-closed <code>&lt;tbody&gt;</code> element, its opening tag can be omitted.

**父标签**


		Within the required parent <a href="/en-US/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> element,
		the <code>&lt;tbody&gt;</code> element can be added after a
		<a href="/en-US/docs/Web/HTML/Element/caption"><code>&lt;caption&gt;</code></a>,
		<a href="/en-US/docs/Web/HTML/Element/colgroup"><code>&lt;colgroup&gt;</code></a>, and a
		<a href="/en-US/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a> element.
	  

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)


---

## 属性

 包含全局属性: true

- align

	
	<p>This enumerated attribute specifies how horizontal alignment of each cell content will be handled. Possible values are:</p>
	<ul>
	  <li><code>left</code>, aligning the content to the left of the cell</li>
	  <li><code>center</code>, centering the content in the cell</li>
	  <li><code>right</code>, aligning the content to the right of the cell</li>
	  <li><code>justify</code>, inserting spaces into the textual content so that the content is justified in the cell</li>
	  <li><code>char</code>, aligning the textual content on a special character with a minimal offset, defined by the <a aria-current="page" href="/en-US/docs/Web/HTML/Element/tbody#attr-char"><code>char</code></a> and <a aria-current="page" href="/en-US/docs/Web/HTML/Element/tbody#attr-charoff"><code>charoff</code></a> attributes.</li>
	</ul>
	<p>If this attribute is not set, the <code>left</code> value is assumed.</p>
	<p>As this attribute is deprecated, use the CSS <a href="/en-US/docs/Web/CSS/text-align"><code>text-align</code></a> property instead.</p>
	<div id="sect1" class="notecard note">
	  <p><strong>Note:</strong> The equivalent <code>text-align</code> property for the <code>align="char"</code> is not implemented in any browsers yet. See the <a href="/en-US/docs/Web/CSS/text-align#browser_compatibility"><code>text-align</code>'s browser compatibility section</a> for the <code>&lt;string&gt;</code> value.</p>
	</div>
  
- bgcolor

	
	<p>The background color of the table. It is a <a href="/en-US/docs/Web/CSS/color_value#rgb_colors">6-digit hexadecimal RGB code</a>, prefixed by a '<code>#</code>'. One of the predefined <a href="/en-US/docs/Web/CSS/color_value#color_keywords">color keywords</a> can also be used.</p>
	<p>As this attribute is deprecated, use the CSS <a href="/en-US/docs/Web/CSS/background-color"><code>background-color</code></a> property instead.</p>
  
- char

	
	<p>This attribute is used to set the character to align the cells in a column on. Typical values for this include a period (<code>.</code>) when attempting to align numbers or monetary values. If <a href="/en-US/docs/Web/HTML/Element/tbody#attr-align" aria-current="page"><code>align</code></a> is not set to <code>char</code>, this attribute is ignored.</p>
  
- charoff

	
	<p>This attribute is used to indicate the number of characters to offset the column data from the alignment characters specified by the <code>char</code> attribute.</p>
  
- valign

	
	<p>This attribute specifies the vertical alignment of the text within each row of cells of the table header. Possible values for this attribute are:</p>
	<ul>
	  <li><code>baseline</code>, which will put the text as close to the bottom of the cell as it is possible, but align it on the <a href="https://en.wikipedia.org/wiki/Baseline_%28typography%29" class="external" rel=" noopener">baseline</a> of the characters instead of the bottom of them. If characters are all of the size, this has the same effect as <code>bottom</code>.</li>
	  <li><code>bottom</code>, which will put the text as close to the bottom of the cell as it is possible;</li>
	  <li><code>middle</code>, which will center the text in the cell;</li>
	  <li>and <code>top</code>, which will put the text as close to the top of the cell as it is possible.</li>
	</ul>
	<p>As this attribute is deprecated, use the CSS <a href="/en-US/docs/Web/CSS/vertical-align"><code>vertical-align</code></a> property instead.</p>
  "#####
}
