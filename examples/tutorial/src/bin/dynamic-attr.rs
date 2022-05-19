use rtml::{
    attr,
    tags::{html, img, Html},
};

#[rtml::page]
fn adding_data() -> Html {
    let src = "";
    html(img(attr! { src = src }))
}
