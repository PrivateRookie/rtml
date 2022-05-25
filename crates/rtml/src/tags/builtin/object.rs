def! {
  object,
  Object,
  ObjectArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<object>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/object)

---

**允许的内容**

<dfn>允许内容</dfn> zero or more <a href="/zh-CN/docs/Web/HTML/Element/param"><code>&lt;param&gt;</code></a> elements, then <a href="/en-US/docs/Web/Guide/HTML/Content_categories#transparent_content_models" title="HTML/Content categories#Transparent content models">Transparent content</a>.

**是否可忽略关闭标签**

<dfn>标签闭合</dfn> 不允许，开始标签和结束标签都不能省略。

**父标签**

<dfn>允许的父级元素</dfn> Any element that accepts <a href="/en-US/docs/Web/Guide/HTML/Content_categories#embedded_content" title="HTML/Content categories#Embedded content">embedded content</a>.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement)


---

## 属性

 包含全局属性: true

- archive

	用来指名对象资源列表的以空格分隔的 URI 列表。
- border

	元素周围的边框的宽度，单位为像素。
- classid

	对象实现的 URI，可以同时与 <strong>data</strong> 属性使用，或者使用 <strong>data</strong> 属性替代。
- codebase

	解析 <strong>classid</strong>，<strong>data</strong> 或者 <strong>archive</strong> 中定义的相对路径的根路径，如果没有定义，默认为当前文档的 base URI。
- codetype

	<strong>classid</strong> 定义的 data 的内容类型。
- data

	一个合法的 URL 作为资源的地址，，需要为 <strong>data</strong> 和 <strong>type </strong>中至少一个设置值。
- declare

	取值为布尔的属性可以设置这个元素为仅声明的格式。对象必须被随后的 <code>&lt;object&gt; 元素实例化。在</code> HTML5 中，完整的重复 &lt;object&gt; 元素，可以重用元素。
- form

	对象元素关联的 form 元素（属于的 form）。 取值必须是同一文档下的一个 <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> 元素的 ID。
- height

	资源显示的高度，单位是 CSS 像素。
- name

	浏览上下文名称（HTML5），或者控件名称（HTML 4）。
- standby

	对象的实现和数据加载过程中，浏览器可以显示的信息。
- tabindex

	当前元素在文档 Tab 导航中的顺序。
- type

	<strong>data</strong> 指定的资源的 MIME 类型，需要为 <strong>data</strong> 和 <strong>type </strong>中至少一个设置值。
- usemap

	指向一个 <a href="/zh-CN/docs/Web/HTML/Element/map"><code>&lt;map&gt;</code></a> 元素的 hash-name；格式为 ‘#’ 加 map 元素 <a href="/zh-CN/docs/Web/HTML/Element/map#attr-name"><code>name</code></a> 元素的值。
- width

	资源显示的宽度，单位是 CSS 像素。"#####
}
