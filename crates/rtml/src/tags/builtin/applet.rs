def! {
  applet,
  Applet,
  AppletArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<applet>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/applet)

---

**允许的内容**



**是否可忽略关闭标签**



**父标签**



[Dom API]()


---

## 属性

 包含全局属性: false

- align

	该属性被用来定义网页上 applet 相对于周围内容的对齐方式。HTML4.01 规范定义了 bottom, left, middle, right 和 top 的值，而 Microsoft（微软）和 Netscaple（网景）也可能支持<strong>absbottom</strong>,&nbsp;<strong>absmiddle</strong>,&nbsp;<strong>baseline</strong>,&nbsp;<strong>center</strong>和<strong>texttop。</strong>
- alt

	该属性造成在不支持 Java 的浏览器上显示出一段替代的描述性文字。&nbsp;页面设计者应该记住在<code>&lt;applet&gt;</code> 元素中封闭的内容也可以呈现为替代性文本。
- archive

	该属性涉及到 applet 的存档或压缩版本及其相关类文件，这可能会减少下载时间。
- code

	该属性指定被加载和执行的 applet 类文件的 URL。Applet 文件名由一个.class 文件扩展名定义。由 code 指定的 URL 可能与 codebase 属性有关。
- codebase

	该属性给出绝对或相对的，由 code 属性引用的 applet 的.class 文件储存的目录的 URL。
- datafld

	该属性支持 Internet Explorer4 及更高的版本，指定提供边界数据的数据源对象的列名。该属性可以用来指定各种传递到 Java 的<a href="/zh-CN/docs/Web/HTML/Element/param"><code>&lt;param&gt;</code></a> 元素。
- datasrc

	就像 datafld，该属性被用于 Internet Explorer 4 版本以下的数据绑定（data binding）。它指明了数据源对象的 id，这个数据对象提供了被与 applet 有关的元素约束的数据。
- height

	该属性提供了 applet 所需的高度，以像素为单位。
- hspace

	该属性指定了保存在 applet 两侧的额外横向空间，以像素为单位。
- mayscript

	在 Netscape 中，该属性允许使用在文档中嵌入的脚本语言程序访问 applet。
- name

	该属性为 applet 分配一个名称，以便它可以被其他资源识别，尤其是脚本语言。
- object

	该属性指定一个序列化表示的 applet 的 URL。
- src

	为 Internet Explorer 4 及更高版本制定，该属性为 applet 相关文件指定一个 URL。 该定义及使用是不明确的，也不属于 HTML 标准。
- vspace

	该属性指定了保存在 applet 以上或以下的额外垂直空间，以像素为单位。
- width

	该属性指定了 applet 所需的宽度，以像素为单位。"#####
}

