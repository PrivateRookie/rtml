def! {
  th,
  Th,
  ThArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<th>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/th)

---

**允许的内容**
	<div class="content-models" id="sect1">
	<div id="table-mdls">流内容（除 header、footer、sectioning content 或 heading content 的继承。）</div>
	</div>
   

**是否可忽略关闭标签**

开始标签是必需要的，而结束标签有时可以省略：当其后是<a href="/zh-CN/docs/Web/HTML/Element/th" aria-current="page"><code>&lt;th&gt;</code></a> 或 <a href="/zh-CN/docs/Web/HTML/Element/td"><code>&lt;td&gt;</code></a> ，或者其后没有其他数据内容在其父元素内，。

**父标签**

&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/tr"><code>&lt;tr&gt;</code></a> 元素

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)


---

## 属性

包含全局属性: true


- colspan

	这个属性包含一个正整数表示了每单元格中扩展列的数量。默认值为<code>1</code>&nbsp;。超过 1000 的值被视作 1000。
- headers

	这个属性包含了一个空间分隔的字符串的列表，每个与其他<a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/th"><code>&lt;th&gt;</code></a>元素相关联的<code>id</code>&nbsp;属性一一对应。
- rowspan

	这个属性包含一个正整数表示了每单元格中扩展列的数量。默认值为<code>1.</code> 如果该值被设置为&nbsp;<code>0</code>, 这个单元格就被扩展为 (<a href="/zh-CN/docs/Web/HTML/Element/thead"><code>&lt;thead&gt;</code></a>，<a class="only-in-en-us" title="Currently only available in English (US)" href="/en-US/docs/Web/HTML/Element/tbody"><code>&lt;tbody&gt;</code> <small>(en-US)</small></a> 或<a href="/zh-CN/docs/Web/HTML/Element/tfoot"><code>&lt;tfoot&gt;</code></a>) 中表格部分的最后一个元素。比 65534 大的值被视作 65534。
- scope

	这个枚举属性定义了表头元素 <sup>(在<a href="/zh-CN/docs/Web/HTML/Element/th" aria-current="page"><code>&lt;th&gt;</code></a>中定义) </sup>关联的单元格。它可能有以下值：
	<ul>
		<li><code>row</code>:&nbsp; 表头关联一行中所有的单元格。</li>
		<li><code>col</code>: 表头关联一列中所有的单元格。</li>
		<li><code>rowgroup</code>:表头属于一个行组并与其中所有单元格相关联。这些单元格可以被放在表头的左侧或右侧，取决于 <a href="/zh-CN/docs/Web/HTML/Element/table"><code>&lt;table&gt;</code></a> 元素中&nbsp;<code><a href="/en-US/docs/Web/HTML/Global_attributes/dir">dir</a></code> 属性的值 。</li>
		<li><code>colgroup</code>: 表头属于一个列组并与其中所有单元格相关联。</li>
		<li><code>auto</code></li>
	</ul>

 "#####
}
