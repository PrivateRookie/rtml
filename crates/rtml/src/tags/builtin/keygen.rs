def_tag! {
  keygen,
  Keygen,
  KeygenArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<keygen>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/keygen)

---

**允许的内容**

None, it is an <a href="/zh-CN/docs/Glossary/Empty_element">empty element</a>.

**是否可忽略关闭标签**

Must have a start tag and must not have an end tag.

**父标签**

Any element that accepts <a title="HTML/Content_categories#Phrasing_content" href="/en-US/docs/Web/Guide/HTML/Content_categories#phrasing_content">phrasing content</a>.

[Dom API](https://developer.mozilla.org/en-US/docs/Web/API/HTMLKeygenElement)


---

## 属性

 包含全局属性: true

- autofocus

	This Boolean attribute lets you specify that the control should have input focus when the page loads, unless the user overrides it, for example by typing in a different control. Only one form element in a document can have the <code>autofocus</code> attribute, which is a Boolean.
- challenge

	A challenge string that is submitted along with the public key. Defaults to an empty string if not specified.
- disabled

	This Boolean attribute indicates that the form control is not available for interaction.
- form

	The form element that this element is associated with (its <em>form owner</em>). The value of the attribute must be an <code>id</code> of a <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> element in the same document. If this attribute is not specified, this element must be a descendant of a <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> element. This attribute enables you to place <code>&lt;keygen&gt; </code>elements anywhere within a document, not just as descendants of their form elements.
- keytype

	The type of key generated. The default value is <code>RSA</code>.
- name

	The name of the control, which is submitted with the form data."#####
}
