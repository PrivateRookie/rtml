def_tag! {
  marquee,
  Marquee,
  MarqueeArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<marquee>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/marquee)

---

**允许的内容**



**是否可忽略关闭标签**



**父标签**



[Dom API]()


---

## 属性

 包含全局属性: false

- behavior

	设置文本在 marquee 元素内如何滚动。可选值有&nbsp;<code>scroll，</code><code>slide</code> 和&nbsp;<code>alternate。</code>&nbsp;如果未指定值，默认值为&nbsp;<code>scroll。</code>
- bgcolor

	通过颜色名称或十六进制值设置背景颜色。
- direction

	设置 marquee 内文本滚动的方向。可选值有&nbsp;<code>left</code>, <code>right</code>, <code>up</code> and <code>down。</code>如果未指定值，默认值为&nbsp;<code>left。</code>
- height

	以像素或百分比值设置高度。
- hspace

	设置水平边距。
- loop

	设置 marquee 滚动的次数。如果未指定值，默认值为&nbsp;−1，表示 marquee 将连续滚动。
- onbounce

	当 marquee 滚动到结尾时触发。它只能在 behavior 属性设置为 alternate 时触发。
- onfinish

	当 marquee 完成 loop 属性设置的值时触发。它只能在 loop 属性设置为大于 0 的某个数字时触发。
- onstart

	当 marquee 开始滚动时触发。
- scrollamount

	设置每次滚动时移动的长度（以像素为单位）。默认值为 6。
- scrolldelay

	设置每次滚动时的时间间隔（以毫秒为单位）。默认值为&nbsp;85。请注意，&nbsp;除非指定 truespeed 值，否则将忽略任何小于 60 的值，并改为使用 60。
- truespeed

	默认情况下，会忽略小于 60 的 scrolldelay 值。如果存在 truespeed，那些值不会被忽略。
- vspace

	以像素或百分比值设置垂直边距。
- width

	以像素或百分比值设置宽度。"#####
}
