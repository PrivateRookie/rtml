use rtml::tags::{h1, html};

fn main() {
    let page = html(h1("hello world!"));
    println!("{}", page);
}
