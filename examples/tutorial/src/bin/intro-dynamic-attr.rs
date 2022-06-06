use rtml::{
    t_attr,
    tags::{html, img, Html},
};

fn dynamic_attr() -> Html {
    let src = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
    html(img(t_attr! { src = src }))
}

fn main() {
    println!("{}", dynamic_attr())
}
