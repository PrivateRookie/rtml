def! {
  source,
  Source,
  SourceArg,
  doc:
  "zh-CN" = r#####"`<source>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/source)

---

**允许的内容**

A <a href="/zh-CN/docs/Web/HTML/Element/picture"><code>&lt;picture&gt;</code></a> element, and it should be placed&nbsp;before the <a href="/zh-CN/docs/Web/HTML/Element/img"><code>&lt;img&gt;</code></a> element.

**是否可忽略关闭标签**

None.

**父标签**

None, it is an <a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement)


---

## 属性

 包含全局属性: true

- media

	<a class="internal" href="/en-US/docs/Web/CSS/Media_Queries/Using_media_queries">Media query</a> of the resource's intended media; this should be used only in a <a href="/zh-CN/docs/Web/HTML/Element/picture"><code>&lt;picture&gt;</code></a> element.
- sizes

	Is a list of source sizes that describes the final rendered width of the image represented by the source. Each source size consists of a comma-separated list of media condition-length pairs. This information is used by the browser to determine, before laying the page out, which image defined in <a href="/zh-CN/docs/Web/HTML/Element/source#attr-srcset" aria-current="page"><code>srcset</code></a> to use.<br>
 The <code>sizes</code> attribute has an effect only when the <a href="/zh-CN/docs/Web/HTML/Element/source" aria-current="page"><code>&lt;source&gt;</code></a> element is the direct child of a <a href="/zh-CN/docs/Web/HTML/Element/picture"><code>&lt;picture&gt;</code></a> element.
- src

	Required for <a href="/zh-CN/docs/Web/HTML/Element/audio"><code>&lt;audio&gt;</code></a> and <a href="/zh-CN/docs/Web/HTML/Element/video"><code>&lt;video&gt;</code></a>, address of the media resource. The value of this attribute is ignored when the <code>&lt;source&gt;</code> element is placed inside a <a href="/zh-CN/docs/Web/HTML/Element/picture"><code>&lt;picture&gt;</code></a> element.
- srcset

	A list of one or more strings separated by commas indicating a set of possible images represented by the source for the browser to use. Each string is composed of:
 <ol>
  <li>one URL to an image,</li>
  <li>a width descriptor, that is a positive integer directly followed by <code>'w'</code>. The default value, if missing, is the infinity.</li>
  <li>a pixel density descriptor, that is a positive floating number directly followed by <code>'x'</code>. The default value, if missing, is <code>1x</code>.</li>
 </ol>

 <p>Each string in the list must have at least a width descriptor or a pixel density descriptor to be valid. Among the list, there must be only one string containing the same tuple of width descriptor and pixel density descriptor.<br>
  The browser chooses the most adequate image to display at a given point of time.<br>
  The <code>srcset</code> attribute has an effect only when the <a aria-current="page" href="/zh-CN/docs/Web/HTML/Element/source"><code>&lt;source&gt;</code></a> element is the direct child of a <a href="/zh-CN/docs/Web/HTML/Element/picture"><code>&lt;picture&gt;</code></a> element.</p>
 
- type

	The MIME-type of the resource, optionally with a <code>codecs</code> parameter. See <a href="https://tools.ietf.org/html/rfc4281" rel=" noopener" class="external">RFC 4281</a> for information about how to specify codecs."#####
}

