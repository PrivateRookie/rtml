use rtml::{
    attr,
    tags::{html, img, Html},
};

#[rtml::page]
fn dynamic_attr() -> Html {
    let src = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
    html(img(attr! { src = src }))
}
