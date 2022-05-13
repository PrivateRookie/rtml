#[derive(Debug, Clone)]
pub struct TagFormatter {
    /// tab stop size
    pub tab_size: usize,
    /// init ident
    pub indent: usize,
    /// line if tag has attrs
    /// 
    /// when false
    /// ```html
    /// <div id="nice">...</div>
    /// ```
    /// 
    /// when true
    /// ```html
    /// <div
    ///  id="nice"
    ///  >
    ///    ...
    /// </div>
    /// ```
    pub newline_on_attr: bool,
    /// new line separator
    pub line_sep: &'static str,
}

impl Default for TagFormatter {
    fn default() -> Self {
        Self {
            tab_size: 4,
            indent: 0,
            newline_on_attr: false,
            line_sep: "\n",
        }
    }
}

impl TagFormatter {
    pub fn pad_size(&self) -> usize {
        self.indent * self.tab_size
    }
}


