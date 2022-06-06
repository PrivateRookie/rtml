def_tag! {
  time,
  Time,
  TimeArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<time>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/time)

---

**允许的内容**



**是否可忽略关闭标签**



**父标签**



[Dom API]()


---

## 属性

 包含全局属性: true

- datetime

	该属性表示此元素的时间和日期，并且属性值必须是一个<a rel=" noopener" href="https://www.w3.org/TR/html5/common-microsyntaxes.html#valid-date-string-with-optional-time" class="external">有效的日期格式，并可包含时间</a>。 如果此值不能被解析为日期，元素不会有一个关联的时间戳。
- pubdate

	(该属性仍在被 WHATWG 和 W3C 组织设计和讨论中.) This Boolean attribute specifies that the date and time given by the element is the publication date of a document. The document it applies to is either the nearest ancestor article element or the document as a whole (if there is no ancestor <a title="zh-cn/HTML/Element/Article" href="/zh-CN/docs/Web/HTML/Element/article">article</a> element). If true, the <code>time</code> element must have a corresponding date. Additionally, each <code>time</code> element indicating a publication date must be the only <code>time</code> element that does so for that document."#####
}
