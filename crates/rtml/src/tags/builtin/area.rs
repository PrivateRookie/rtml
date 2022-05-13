def! {
  area,
  Area,
  AreaArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<area>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/area)

---

**允许的内容**

<dfn>允许的内容</dfn>它是一个空的元素不允许嵌套任何子元素或者文本。

**是否可忽略关闭标签**

<dfn>标签省略</dfn>只能允许有开始标签不允许有结束标签。

**父标签**

<dfn>允许的父元素</dfn> &lt;area&gt;元素必须拥有一个&lt;map&gt;元素祖先元素，但不一定是直接的父元素。

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLAreaElement)


---

## 属性

 包含全局属性: true

- accesskey

	为元素指定一个获取焦点的快捷键。在与指定字符关联的情况下，按 ALT 或类似键选择与该键序列相关联的表单控件。页面设计人员避免使用已经绑定到浏览器的快捷键。这个属性自 HTML5 以来是全局性的。
- alt

	在未显示图像的浏览器上显示代替的文本字符串。这个文本应该能传达给用户与显示图像而没有文本的情况下同等的选择（译者注：就是字和图片表达一个意思）。在 HTML4 中， 这个属性时必需的，但是可以是一个空的串 ("")。在 HTML5 中，这个属性只有在<strong>href</strong> 属性被使用的时候才是必需的。
- coords

	给热点区域设定具体的坐标值。这个值的数值和意义取决于这个值所描述的<strong>形状</strong>属性.。对于矩形或长方形，这个<strong>coords&nbsp;</strong>值为两个 X,Y 对：左上、右下。&nbsp;对于圆形，这个值是&nbsp;<code>x,y,r</code>&nbsp;，这里的&nbsp;<code>x,y</code> 是一对确定圆的中心的坐标而&nbsp;<code>r</code>&nbsp;则表示的是半径值.。对于多边和多边形，这个值是用 x,y 对表示的多边形的每一个点：<code>x1,y1,x2,y2,x3,y3,</code>等等。HTML4 里，&nbsp;值可能是像素数量或者百分比，区别是不是有%出现; HTML5 里，只可能是像素的数量。
- download

	这个属性如果存在的话，表明作者想把超链接用于下载一个资源。请查看<a href="/zh-CN/docs/Web/HTML/Element/a"><code>&lt;a&gt;</code></a> 获得关于 <a href="/zh-CN/docs/Web/HTML/Element/a#attr-download"><code>download</code></a>属性的完整描述。
- href

	area 的超链接，值为一个 URL. HTML4 里，这个值不管是不是有值都要明确指定出来. HTML5 里则不需要。
- hreflang

	指明链接资源的语言类型，值的范围参考<a href="https://www.ietf.org/rfc/bcp/bcp47.txt" class="external" rel=" noopener">BCP47</a>. 这个属性只能在指明 href 属性之后使用。
- media

	指明链接资源的媒体类型，例：print and screen。如果此属性省略，默认全部。仅在 href 属性存在情况下使用。
- name

	为可点击区域定义一个名字以使旧浏览器解析。
- nohref

	指明此区域没有超链接。在&lt;area&gt;中必须存在 nohref 或者 href。
	<div class="note notecard" id="sect1">
	<p><strong>用法提示：</strong>此属性在 html5 中是废弃的，而忽略 href 属性就足够了。</p>
	</div>
 
- rel

	对于包含 href 属性的锚，该属性指定目标对象与链接对象的关系。该值是一个逗号分隔的链接类型值列表。这些值及其语义将由一些可能对文档作者有意义的权威进行注册。如果没有其他的关系，默认的关系是无效的。只有当 href 属性是 presen 时才使用该属性
- shape

	相关联的热点的形状。HTML 5 和 HTML 4 的规范定义了值 rect，它定义了一个矩形区域;圆圈，它定义了一个圆形区域;多边形，它定义了一个多边形;默认情况下，这表示整个区域超出了任何定义的形状。许多浏览器，特别是 Internet Explorer 4 和更高版本，支持弧形、多边形和矩形作为形状的有效值;这些值{ { Non-standard_inline } }。
- tabindex

	用于指定浏览器 tab 键获取焦点的顺序。在 html5 中是全局属性。
- target

	本属性指明了在什么地方显示链接的资源. HTML4 里，这个值是一个 frame 的链接或者关键字. HTML5 里，它是一个浏览器上下文 (比如：标签，窗口或者内嵌的 frame) 的链接或者关键字。值的含义：
	<ul>
	<li><code>_self</code>: 在当前区域加载链接指向的资源。这个是默认值。</li>
	<li><code>_blank</code>: 在新的未命名的窗口或者 tab 里加载超链接资源。</li>
	<li><code>_parent</code>: 在父级加载超链接资源. HTML4 里，是当前 frame 的父级，HTML5 里是当前的浏览器上下文，如果当前环境没有父级，行为和<code>_self</code>一样。</li>
	<li><code>_top</code>: HTML4 里：将响应加载到完整的原始窗口中，取消所有其他帧。在 HTML5 中：将响应加载到顶级浏览上下文 (也就是说，浏览上下文是当前版本的祖先，并且没有父类)。如果没有父类，这个选项的行为方式与 self 相同</li>
	</ul>
	本属性只能在指明 href 属性之后使用。
- type

	该属性指定了用于链接目标的 MIME 类型的媒体类型。一般来说，这是严格的咨询信息;然而，在未来，浏览器可能会为多媒体类型添加一个小图标。例如，当类型设置为音频/wav 时，浏览器可能会添加一个小的扬声器图标。公认的 MIME 类型的完整列表，请参阅 <http://www.w3.org/TR/html4/references.html> ref-MIMETYPES。只有当 href 属性存在时才使用该属性。"#####
}

