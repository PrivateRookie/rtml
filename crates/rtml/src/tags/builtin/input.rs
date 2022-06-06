def_tag! {
  input,
  Input,
  InputArg,
  doc:
  "en-US" = "";
  "zh-CN" = r#####"`<input>` [doc](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Element/input)

---

**允许的内容**

无，这是一个<a href="/zh-CN/docs/Glossary/Empty_element">空元素</a>。

**是否可忽略关闭标签**


<p>必须有开始标签但不必有结束标签。</p>
   

**父标签**

任何元素都可以包含语句型元素。

[Dom API](https://developer.mozilla.org/zh-CN/docs/Web/API/HTMLInputElement)


---

## 属性

    包含全局属性: true

- &lt;input&gt;

	Specifies an "action hint" used to determine how to label the enter key on mobile devices with virtual keyboards. Supported values are <code>go</code>, <code>done</code>, <code>next</code>, <code>search</code>, and <code>send</code>; these automatically get mapped to the appropriate string (and are case-insensitive).
- accept

	如果该元素的 <strong>type</strong> 属性的值<code>是 file</code>,则该属性表明了服务器端可接受的文件类型；否则它将被忽略。该属性的值必须为一个逗号分割的列表，包含了多个唯一的内容类型声明：
    <ul>
    <li>以 STOP 字符 (U+002E) 开始的文件扩展名。（例如：".jpg,.png,.doc"）</li>
    <li>一个有效的 MIME 类型，但没有扩展名</li>
    <li><code>audio/*</code> 表示音频文件 <span class="badge inline html-version"><a title="This is a link to an unwritten page" href="/zh-CN/docs/HTML/HTML5" class="page-not-created">HTML5</a></span></li>
    <li><code>video/*</code> 表示视频文件 <span class="badge inline html-version"><a class="page-not-created" title="This is a link to an unwritten page" href="/zh-CN/docs/HTML/HTML5">HTML5</a></span></li>
    <li><code>image/*</code> 表示图片文件 <span class="badge inline html-version"><a href="/zh-CN/docs/HTML/HTML5" class="page-not-created" title="This is a link to an unwritten page">HTML5</a></span></li>
    </ul>
 
- accesskey

	用户按下后可以获得此控件焦点的单个字符。这是 HTML5 全局属性。
- autocapitalize

	Specifies an "action hint" used to determine how to label the enter key on mobile devices with virtual keyboards. Supported values are&nbsp;<code>go</code>,&nbsp;<code>done</code>,&nbsp;<code>next</code>,&nbsp;<code>search</code>, and&nbsp;<code>send</code>. These automatically get mapped to the appropriate string and are case-insensitive.
- autocomplete

	这个属性表示这个控件的值是否可被浏览器自动填充。如果<strong>type</strong>属性的值是 hidden、checkbox、radio、file，或为按钮类型（button、submit、reset、image），则本属性被忽略。可用的值是：
    <ul>
    <li><code>off</code>: 用户必须手动填值，或者该页面提供了自己的自动补全方法。浏览器不对此字段自动填充。</li>
    <li><code>on</code>: 浏览器可以根据用户先前的填表情况对此字段自动填值。</li>
    <li><code>name</code>: 完整的姓名</li>
    <li><code>honorific-prefix:&nbsp;</code>Prefix or title (e.g. "Mr.", "Ms.", "Dr.", "Mlle")</li>
    <li><code>given-name&nbsp;</code>：名</li>
    <li><code>additional-name</code></li>
    <li><code>family-name</code>：姓</li>
    <li><code>honorific-suffix</code>:&nbsp;Suffix (e.g. "Jr.", "B.Sc.", "MBASW", "II")</li>
    <li><code>nickname</code></li>
    <li><code>email</code></li>
    <li><code>username</code></li>
    <li><code>new-password</code>: 新密码（如创建帐号或更改密码时使用）</li>
    <li><code>current-password</code></li>
    <li><code>organization-title</code>:&nbsp;Job title (e.g. "Software Engineer", "Senior Vice President", "Deputy Managing Director")</li>
    <li><code>organization</code></li>
    <li><code>street-address</code></li>
    <li><code>address-line1,&nbsp;<code>address-line2,&nbsp;<code>address-line3,&nbsp;<code>address-level4,&nbsp;<code>address-level3,&nbsp;<code>address-level2,&nbsp;<code>address-level1</code></code></code></code></code></code></code></li>
    <li><code>country</code></li>
    <li><code>country-name</code></li>
    <li><code>postal-code</code></li>
    <li><code>cc-name</code>:&nbsp;Full name as given on the payment instrument</li>
    <li><code>cc-given-name</code></li>
    <li><code>cc-additional-name</code></li>
    <li><code>cc-family-name</code></li>
    <li><code>cc-number</code>:&nbsp;Code identifying the payment instrument (e.g. the credit card number)</li>
    <li><code>cc-exp:</code>&nbsp;Expiration date of the payment instrument</li>
    <li><code>cc-exp-month</code></li>
    <li><code>cc-exp-year</code></li>
    <li><code>cc-csc</code>:&nbsp;Security code for the payment instrument&nbsp;</li>
    <li><code>cc-type</code>:&nbsp;Type of payment instrument (e.g.&nbsp;Visa)</li>
    <li><code>transaction-currency</code></li>
    <li><code>transaction-amount</code></li>
    <li><code>language</code>:&nbsp;Preferred language;&nbsp;Valid BCP 47 language tag</li>
    <li><code>bday</code></li>
    <li><code>bday-day</code></li>
    <li><code>bday-month</code></li>
    <li><code>bday-year</code></li>
    <li><code>sex</code>:&nbsp;Gender identity (e.g. Female, Fa'afafine);&nbsp;Free-form text, no newlines</li>
    <li><code>tel</code></li>
    <li><code>url</code>:&nbsp;Home page or other Web page corresponding to the company, person, address, or contact information in the other fields associated with this field</li>
    <li><code>photo</code>:&nbsp;Photograph, icon, or other image corresponding to the company, person, address, or contact information in the other fields associated with this field</li>
    <li>
    <p>参考 <a rel=" noopener" class="external" href="https://html.spec.whatwg.org/multipage/forms.html#autofill">WHATWG 标准</a> 获取更多详细内容。</p>
    </li>
    </ul>

    <p>如果<code>&lt;input&gt;</code>元素上没有<strong>autocomplete</strong>属性，浏览器可使用包含该 input 元素的表单（<code>&lt;form&gt;</code>）或通过 input 的<strong>form</strong>属性指定的表单的<strong>autocomplete</strong>属性值。更多信息请参见<a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a>的<code>autocomplete</code>属性。</p>

    <p>与其他浏览器不同，<strong>autocomplete </strong>还控制着 Firefox 浏览器 对 &lt;input&gt;&nbsp;持久化动态禁用状态和（如果适用）跨页面加载的动态检查。持久化特性默认是开启的。设置<strong>autocomplete</strong>的值为<strong>off </strong>可以关闭该特性<strong>。</strong>即使 autocomplete 属性通常不应用于&lt;input&gt;的 type，它也可以工作。具体可以查看<a href="https://bugzilla.mozilla.org/show_bug.cgi?id=654072" class="external" rel=" noopener">bug&nbsp;654072</a>。</p>
 
- autocorrect

	This is a nonstandard attribute used by iOS Safari Mobile which controls whether and how the text value should be automatically capitalized as it is entered/edited by the user. The non-deprecated values are available in iOS 5 and later. Possible values are:
    <ul>
    <li><code>none</code>: Completely disables automatic capitalization</li>
    <li><code>sentences</code>: Automatically capitalize the first letter of sentences.</li>
    <li><code>words</code>: Automatically capitalize the first letter of words.</li>
    <li><code>characters</code>: Automatically capitalize all characters.</li>
    <li><code>on</code>: <abbr title="Deprecated. Not for use in new websites." class="icon icon-deprecated">
        <span class="visually-hidden">Deprecated</span>
    </abbr> Deprecated since iOS 5.</li>
    <li><code>off</code>: <abbr class="icon icon-deprecated" title="Deprecated. Not for use in new websites.">
        <span class="visually-hidden">Deprecated</span>
    </abbr> Deprecated since iOS 5.</li>
    </ul>
    <a href="https://developer.apple.com/library/safari/documentation/AppleApplications/Reference/SafariHTMLRef/Articles/Attributes.html#//apple_ref/doc/uid/TP40008058-autocapitalize" rel=" noopener" class="external"><code>autocapitalize</code> documentation in the Safari HTML Reference</a>
- autofocus

	这个布尔属性允许您指定的表单控件在页面加载时具有焦点（自动获得焦点），除非用户将其覆盖，例如通过键入不同的控件。文档中只有一个表单元素可以具有 autofocus 属性，它是一个布尔值。&nbsp;如果 type 属性设置为隐藏则不能应用（即您不能自动获得焦点的属性设置为隐藏的控件）。
- capture
    <p>Introduced in the HTML Media Capture specification and valid for the&nbsp;<code>file</code>&nbsp;input type only, the&nbsp;<code>capture</code>&nbsp;attribute defines which media—microphone, video, or camera—should be used to capture a new file for upload with&nbsp;<code>file</code>&nbsp;upload control in supporting scenarios. See the&nbsp;<a href="/zh-CN/docs/Web/HTML/Element/Input/file">file</a> input type.</p>
 
- checkValidity()
	Selects the entire content of the <code>&lt;input&gt;</code> element, if the element's content is selectable. For elements with no selectable text content (such as a visual color picker or calendar date input), this method does nothing.
- checked

	如果该元素的<strong>type</strong>属性的值为radio 或者checkbox,则该布尔属性的存在与否表明了该控件是否是默认选择状态。
    <p>If present on a&nbsp;<code>checkbox</code>&nbsp;type, it indicates that the checkbox is checked by default (when the page loads). It does&nbsp;<em>not</em>&nbsp;indicate whether this checkbox is currently checked: if the checkbox’s state is changed, this content attribute does&nbsp;not reflect&nbsp;the change. (Only the&nbsp;<a href="/zh-CN/docs/Web/API/HTMLInputElement"><code>HTMLInputElement</code>’s&nbsp;<code>checked</code>&nbsp;IDL attribute</a>&nbsp;is updated.)</p>

    <div class="note notecard" id="sect1">
    <p><strong>Note:</strong> Unlike other input controls, a checkboxes and radio buttons value are only included in the submitted data if they are currently <code>checked</code>. If they are, the name and the value(s) of the checked controls are submitted.</p>

    <p>For example, if a checkbox whose <code>name</code> is <code>fruit</code> has a <code>value</code> of <code>cherry</code>, and the checkbox is checked, the form data submitted will include <code>fruit=cherry</code>. If the checkbox isn't active, it isn't listed in the form data at all. The default <code>value</code> for checkboxes and radio buttons is <code>on</code>.</p>
    </div>
 
- dirname

	
    <p>Valid for&nbsp;<code>text</code>&nbsp;and&nbsp;<code>search</code>&nbsp;input types only, the&nbsp;<code>dirname</code>&nbsp;attribute enables the submission of the directionality of the element. When included, the form control will submit with two name/value pairs: the first being the <a href="#attr-name">name</a> and <a href="#attr-value">value</a>, the second being the value of the&nbsp;<code>dirname</code>&nbsp;as the name with the value of&nbsp;<code>ltr</code>&nbsp;or&nbsp;<code>rtl</code>&nbsp;being set by the browser.</p>

    <pre class="notranslate"><code>&lt;form action="page.html" method="post"&gt;
    &lt;label&gt;Fruit: &lt;input type="text" name="fruit" dirname="fruit.dir" value="cherry"&gt;&lt;/label&gt;
    &lt;input type="submit"/&gt;
    &lt;/form&gt;
    &lt;!-- page.html?fruit=cherry&amp;fruit.dir=ltr --&gt;</code>
    </pre>

    <p>When the form above is submitted, the input cause both the&nbsp;<code>name</code>&nbsp;/&nbsp;<code>value</code>&nbsp;pair of&nbsp;<code>fruit=cherry</code>&nbsp;and the&nbsp;<code>dirname</code>&nbsp;/ direction pair of&nbsp;<code>fruit.dir=ltr</code>&nbsp;to be sent.</p>
 
- disabled

	这个布尔属性表示此表单控件不可用。 特别是在禁用的控件中， <code>click</code> 事件 <a class="external" href="https://www.whatwg.org/specs/web-apps/current-work/multipage/association-of-controls-and-forms.html#enabling-and-disabling-form-controls" rel=" noopener">将不会被分发</a> 。 并且，禁用的控件的值在提交表单时也不会被提交。如果 <strong>type</strong> 属性为&nbsp; hidden，此属性将被忽略。
- form

	
    <p>A string specifying the <a href="/zh-CN/docs/Web/HTML/Element/form"><code>&lt;form&gt;</code></a> element with which the input is associated (that is, its&nbsp;<strong>form owner</strong>). This string's value, if present, must match the <a href="/zh-CN/docs/Web/HTML/Global_attributes#attr-id"><code>id</code></a> of a&nbsp;<code>&lt;form&gt;</code>&nbsp;element in the same document. If this attribute isn't specified, the&nbsp;<code>&lt;input&gt;</code>&nbsp;element is associated with the nearest containing form, if any.</p>

    <p>The&nbsp;<code>form</code>&nbsp;attribute lets you place an input anywhere in the document but have it included with a form elsewhere in the document.</p>

    <p>Note:&nbsp;An input can only be associated with one form.</p>
 
- formaction

	
    <p>Valid for the&nbsp;<code>image</code>&nbsp;and&nbsp;<code>submit</code>&nbsp;input types only. See the <a href="/zh-CN/docs/Web/HTML/Element/Input/submit">submit</a> input type for more information.</p>
 
- formenctype

	
    <p>Valid for the&nbsp;<code>image</code>&nbsp;and&nbsp;<code>submit</code>&nbsp;input types only. See the <a href="/zh-CN/docs/Web/HTML/Element/Input/submit">submit</a> input type for more information.</p>
 
- formmethod

	
    <p>Valid for the&nbsp;<code>image</code>&nbsp;and&nbsp;<code>submit</code>&nbsp;input types only. See the <a href="/zh-CN/docs/Web/HTML/Element/Input/submit">submit</a> input type for more information.</p>
 
- formnovalidate

	
    <p>Valid for the&nbsp;<code>image</code>&nbsp;and&nbsp;<code>submit</code>&nbsp;input types only. See the <a href="/zh-CN/docs/Web/HTML/Element/Input/submit">submit</a> input type for more information.</p>
 
- formtarget

	
    <p>Valid for the&nbsp;<code>image</code>&nbsp;and&nbsp;<code>submit</code>&nbsp;input types only. See the <a href="/zh-CN/docs/Web/HTML/Element/Input/submit">submit</a> input type for more information.</p>
 
- height

	如果<strong>type</strong>属性的值是image，这个属性定义了按钮图片的高度。
- id
    <p>Global attribute valid for all elements, including all the input types, it defines a unique identifier (ID) which must be unique in the whole document. Its purpose is to identify the element when linking. The value is used as the value of the <a href="/zh-CN/docs/Web/HTML/Element/label"><code>&lt;label&gt;</code></a>'s&nbsp;<code>for</code>&nbsp;attribute to link the label with the form control. See the <a href="#labels">the label element</a> below.</p>
 
- incremental

	This is a nonstandard attribute supported by Safari that only applies when the&nbsp;<strong>type</strong>&nbsp;is&nbsp;<code>search</code>. It is used to control the maximum number of entries that should be displayed in the&nbsp;<code>&lt;input&gt;</code>'s native dropdown list of past search queries. Its value should be a nonnegative decimal integer.
- inputmode

	
    <p>Global value valid for all elements, it provides a hint to browsers as to the type of virtual keyboard configuration to use when editing this element or its contents. Values include none<br>
    <code>text</code>,&nbsp;<code>tel</code>,&nbsp;<code>url</code>,&nbsp;<code>email</code>,&nbsp;<code>numeric</code>,&nbsp;<code>decimal</code>, and&nbsp;<code>search</code>.</p>
 
- list

	此项目的最大（数字或日期时间）值，且不得小于其最小值（<strong>min</strong>属性）值。
- max

	如果 <strong>type</strong>&nbsp;的值是&nbsp; text,&nbsp;email,&nbsp;search, password, tel, 或&nbsp;url，那么这个属性指明了用户最多可以输入的字符个数（按照 Unicode 编码方式计数）；对于其他类型的输入框，该属性被忽略。它可以大于&nbsp;<strong>size</strong> 属性的值。如果不指定这个属性，那么用户可以输入任意多的字符。如果指定为一个负值，那么元素表现出默认行为，即用户可以输入任意多的字符。本属性的约束规则，仅在元素的 value 属性发生变化时才会执行。译者注:ie10+
- maxlength

	此项目的最小（数字或日期时间）值，且不得大于其最大值（最大属性）值。
- min

	
    <p>Valid for&nbsp;<code>text</code>,&nbsp;<code>search</code>,&nbsp;<code>url</code>,&nbsp;<code>tel</code>,&nbsp;<code>email</code>, and&nbsp;<code>password</code>, it defines the minimum number of characters (as UTF-16 code units) the user can enter into the entry field. This must be an non-negative integer value smaller than or equal to the value specified by&nbsp;<code>maxlength</code>. If no&nbsp;<code>minlength</code>&nbsp;is specified, or an invalid value is specified, the input has no minimum length.</p>

    <p>The input will fail&nbsp;<a href="/en-US/docs/Web/Guide/HTML/Constraint_validation" class="only-in-en-us" title="Currently only available in English (US)">constraint validation (en-US)</a>&nbsp;if the length of the text entered into the field is fewer than&nbsp;<code>minlength</code>&nbsp;UTF-16 code units long, preventing form submission. See <a href="#client-side_validation">Client-side validation</a> for more information.</p>
 
- minlength

	This Boolean attribute indicates whether the user can enter more than one value.这个属性指示用户能否输入多个值。这个属性仅在<strong>type</strong>属性为 email 或 file 的时候生效&nbsp;; 否则被忽视。
- mozactionhint

	This Boolean attribute indicates if the selector used when the&nbsp;<strong>type</strong>&nbsp;attribute is&nbsp;<code>file</code>has to allow for the selection of directories only.
- multiple

	控件的名称，与表单数据一起提交。
- name

	检查控件值的正则表达式.。pattern 必须匹配整个值，而不仅仅是某些子集.。使用 title 属性来描述帮助用户的模式.。当类型属性的值为text, search, tel, url&nbsp;或&nbsp;email时，此属性适用，否则将被忽略。译者注:ie10+
- pattern

	提示用户输入框的作用。用于提示的占位符文本不能包含回车或换行。仅适用于当<strong>type</strong>&nbsp;属性为text, search, tel, url or email 时;&nbsp;否则会被忽略。
    <div id="sect4" class="note notecard"><strong>Note:</strong>&nbsp;请不要用<code>placeholder</code>&nbsp;属性替换 <a href="/zh-CN/docs/Web/HTML/Element/label"><code>&lt;label&gt;</code></a>&nbsp;元素。他们的作用不同: &nbsp;<a href="/zh-CN/docs/Web/HTML/Element/label"><code>&lt;label&gt;</code></a>&nbsp;属性描述表单元素的角色;&nbsp;也就是说，它展示预期的信息，而<code>placeholder</code>&nbsp;属性是提示用户内容的输入格式。某些情况下&nbsp;<code>placeholder</code>&nbsp;属性对用户不可见，&nbsp;所以当没有它时也需要保证 form 能被理解。</div>
 
- placeholder

	这个布尔属性用于指明用户无法修改控件的值。
 <p><span class="badge inline html-version"><a title="This is a link to an unwritten page" href="/zh-CN/docs/HTML/HTML5" class="page-not-created">HTML5</a></span>&nbsp;如果控件的&nbsp;<strong>type</strong>&nbsp;属性为hidden, range, color, checkbox, radio, file&nbsp;或&nbsp;type 时，此属性会被忽略。</p>
 
- readonly

	这个属性指定用户在提交表单之前必须为该元素填充值。当 type 属性是 hidden,image 或者按钮类型 (submit,reset,button) 时不可使用. <a href="/zh-CN/docs/Web/CSS/:optional"><code>:optional</code></a> 和 <a href="/zh-CN/docs/Web/CSS/:required"><code>:required</code></a> CSS 伪元素的样式将可以被该字段应用作外观。
- reportValidity()

	Sets a custom message to display if the input element's value isn't valid.
- required

	The direction in which selection occurred. This is "forward" if the selection was made from left-to-right in an LTR locale or right-to-left in an RTL&nbsp;locale, or "backward" if the selection was made in the opposite direction. This can be "none"&nbsp;if the selection direction is unknown.
- results

	This Mozilla extension allows you to specify the error message to display when a field doesn't successfully validate.
- select()

	Sets the contents of the specified range of characters in the input element to a given string. A <code>selectMode</code> parameter is available to allow controlling how the existing content is affected.
- selectionDirection

	控件的初始大小。以像素为单位。但当<strong>type</strong>&nbsp; 属性为text&nbsp;或&nbsp;password 时,&nbsp;它表示输入的字符的长度。从 HTML5 开始，&nbsp;此属性仅适用于当&nbsp;<strong>type</strong>&nbsp;属性为&nbsp;text, search, tel, url, email,或&nbsp;password；否则会被忽略。&nbsp;此外，它的值必须大于 0。&nbsp;如果未指定大小，则使用默认值 20。&nbsp;HTML5&nbsp;概述&nbsp;"用户代理应该确保至少大部分字符是可见的",&nbsp;但是不同的字符的用不同的字体表示可能会导致宽度不同。在某些浏览器中，一串带有 x 的字符即使定义了到 x 的大小也将显示不完整。 。
- setCustomValidity()

	Selects the specified range of characters within a textual input element. Does nothing for inputs which aren't presented as text input fields.
- setRangeText()

	Decrements the value of a numeric input by one, by default, or by the specified number of units.
- setSelectionRange()

	Increments the value of a numeric input by one or by the specified number of units.
- size

	将此属性的值设置为<code>true</code>表示元素需要检查其拼写和语法。值<code>default</code> 表示该元素将根据默认行为进行操作，可能基于父元素自己的<code>spellcheck</code>值。值<code>false</code>表示不应该检查元素
- spellcheck

	如果<strong>type</strong>属性的值是image, 这个属性指定了按钮图片的路径; 否则它被忽视。
- src

	使用<strong>min</strong>和<strong>max</strong> 属性来限制可以设置数字或日期时间值的增量。它可以是任意字符串或是正浮点数。如果此属性未设置为任何，则控件仅接受大于最小步长值的倍数的值。
- step

	元素在当前文档的 Tab 导航顺序中的位置。
- tabindex

	
    <p>Global attribute valid for all elements, including all input types, containing a text representing advisory information related to the element it belongs to. Such information can typically, but not necessarily, be presented to the user as a tooltip. The title should NOT be used as the primary explanation of the purpose of the form control. Instead, use the <a href="/zh-CN/docs/Web/HTML/Element/label"><code>&lt;label&gt;</code></a> element with a&nbsp;<code>for</code>&nbsp;attribute set to the form control's <a id="attr-id" href="#attr-id"><b><code>id</code></b></a> attribute. See <a href="#labels">Labels</a> below.</p>
 
- title

	要呈现的控件类型。有关各个类型的信息，请参阅&nbsp;<a href="#<input>_types">Form &lt;input&gt; types</a>，其中包含指向每个类型的更多信息的链接。
- type

	作为图像映射的<a href="/zh-CN/docs/Web/HTML/Element/map"><code>&lt;map&gt;</code></a>元素的名称。
- usemap

	控件的初始值。此属性是可选的，除非<strong>type</strong>&nbsp;属性是<code>radio</code>或<code>checkbox</code>。注意，当重新加载页面时，如果在重新加载之前更改了值，<a href="https://bugzilla.mozilla.org/show_bug.cgi?id=46845#c186" class="link-https external" rel=" noopener">Gecko 和 IE 将忽略 HTML 源代码中指定的值</a>。
- value

	如果<strong>type</strong>属性的值是image，这个属性定义了按钮图片的宽度。
- webkitdirectory

	Immediately runs the validity check on the element, triggering the document to fire the <a href="/en-US/docs/Web/API/HTMLInputElement/invalid_event"><code>invalid</code></a> event at the element if the value isn't valid.
- width

	This is a non-standard attribute supported by Safari that is used to control whether autocorrection should be enabled when the user is entering/editing the text value of the&nbsp;<code>&lt;input&gt;</code>. Possible attribute values are:
    <ul>
    <li><code>on</code>: Enable autocorrection.</li>
    <li><code>off</code>: Disable autocorrection.</li>
    </ul>
    <a rel=" noopener" href="https://developer.apple.com/library/safari/documentation/AppleApplications/Reference/SafariHTMLRef/Articles/Attributes.html#//apple_ref/doc/uid/TP40008058-autocorrect" class="external"><code>autocorrect</code>&nbsp;documentation</a>&nbsp;in the Safari HTML Reference.
- x-moz-errormessage

	Returns <code>true</code> if the element's value passes validity checks; otherwise, returns <code>false</code>."#####
}
