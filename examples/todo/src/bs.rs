use rtml::{
    attr,
    tags::{div, Content, Div},
    Marker,
};

macro_rules! row {
    ($c:expr) => {
        $rtml::tags::div($)
    };
}

pub fn row_div<C: Into<Content>>(content: C) -> Div {
    div((attr! { class = "row" }, content))
}

pub fn col_div<C: Into<Content>, W: Into<Option<usize>>>(width: W, content: C) -> Div {
    let cls = match width.into() {
        Some(w) => {
            format!("col-{w}")
        }
        None => "col".into(),
    };
    div((attr! { class = cls }, content))
}
