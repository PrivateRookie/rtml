def! {
  meter,
  Meter,
  MeterArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<meter>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/meter)

---

**允许的内容**

<a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">Phrasing content</a>, but there must be no <code>&lt;meter&gt;</code> element among its descendants.

**是否可忽略关闭标签**

不允许，开始标签和结束标签都不能省略。

**父标签**

Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement)


---

## 属性

 包含全局属性: true

- form

	该属性将本元素与对应的 form 元素关联。例如，一个计量器可能用来显示某个数值输入框（input 元素，number 类型）的范围。只有当计量器元素被用作表单关联的元素时，该属性才应当被使用；即便如此，如果它作为表单的后代元素出现，它仍然有可能被省略。
- high

	定义了高值区间的下限值。如果设置了，它必须小于最大值，同时必须大于 low 值和最小值。如果没有设置，或者比最大值还大，其值即为最大值。
- low

	定义了低值区间的上限值（译者注：如果 value 介于 min 和 low 之间，该元素就会表现出低值的视觉效果，value 落在 \[min,low\]、\[high,max\] 等不同的区间会使浏览器渲染该元素时出不同的视觉效果）。如果设置了，它必须比最小值属性大，并且不能超过 high 值和最大值。未设置或者比最小值还要小时，其值即为最小值。
- max

	值域的上限边界值。如果设置了，它必须比最小值要大。如果没设置，默认为 1。
- min

	值域的最小边界值。如果设置了，它必须比最大值要小。如果没设置，默认为 0。
- optimum

	这个属性用来指示最优/最佳取值。它必须在正确的值域内（由最小值属性和最大值属性定义）。当使用了 low 和 high 属性时，它指明哪一个取值范围是更好的。例如，假设它介于最小值和 low 之间，那么 lower 区间就被认为是更佳的取值范围。
- value

	当前的数值。如果设置了最小值和最大值（分别由 min 属性和 max 属性定义），它必须介于最小值和最大值之间。如果没有指定或者格式有误，值即为 0.如果给定的值不在最小值和最大值之间，它的值就等于它最接近的一端的值。
 <div class="note notecard" id="sect2"><strong>使用备注：</strong>除非值域在 0 到 1（闭区间）, 否则最小值和最大值属性需要定义，以保证 value 属性在值域范围内。换言之，默认的 min 和 max 值分别为 0 和 1。</div>
 "#####
}
