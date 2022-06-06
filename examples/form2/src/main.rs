use rtml::t_attr;
use rtml::tags::*;

fn index() -> Html {
    html((
        t_attr! {lang="en"},
        (
            head((
                meta(t_attr! { charset="utf-8" }),
                meta(t_attr!{ http-equiv="X-UA-Compatible", content="IE=edge"}),
                meta(t_attr! { name="viewport", content="width=device-width, initial-scale=1.0, maximum-scale=1.0"}),
                meta(t_attr! { name="author", content="Sadegh Rastgoo"}),
                title("LOGIN page-Sadegh Rastgoo"),
                link(t_attr! {
                    rel="stylesheet",
                    href ="https://www.jsdelivr.com/package/npm/normalize.css"
                }),
                link(t_attr! { rel="preconnect", href="https://fonts.gstatic.com"}),
                link(t_attr! { rel="stylesheet", href="https://fonts.googleapis.com/css2?family=Nunito:wght@200;400;700&display=swap"}),
                link(t_attr! { rel="stylesheet", href="https://cdn.bootcdn.net/ajax/libs/font-awesome/6.1.1/css/fontawesome.min.css"}),
                style(include_str!("../assets/index.css"))
            )),
            body((
                div((
                    t_attr! {class="main"},
                    (
                        div((
                            t_attr!{class="card_image-container"},
                            div((
                                t_attr!{class="slide-images"},
                                (
                                    img(t_attr!{class="slide",src="https://i.pinimg.com/564x/54/bb/96/54bb962a32c1093f999cb45d89c9dc0e.jpg",alt="Card Image"}),
                                    img(t_attr!{class="slide",src="https://i.pinimg.com/564x/14/13/38/1413387c9de2609825b61f0719f024d4.jpg",alt="Card Image"}),
                                    img(t_attr!{class="slide",src="https://i.pinimg.com/564x/6a/46/40/6a4640fd2df70f1e76f388175ffb0349.jpg",alt="Card Image"})
                                )
                            ))
                        )),
                        form((
                            t_attr! {action="#",autocomplete="off"},
                            (
                                h2((t_attr!{class="heading-text"},"Sign Up")),
                                div((t_attr!{class="inp-1"},(i(t_attr!{class="fa-thin fa-user"}),input(t_attr!{type="text", name="username",id="inp--username",placeholder="user name"})))),
                                div((t_attr!{class="inp-1"},(i(t_attr!{class="fa-thin fa-envelope"}),input(t_attr!{type="email", name="email",id="inp--email",placeholder="email"})))),
                                div((t_attr!{class="inp-1"},(i(t_attr!{class="fa-thin fa-unlock-keyhole"}),input(t_attr!{type="password", name="password",id="inp--password",placeholder="password"})))),
                                div((
                                    t_attr!{class="inp-2"},
                                    (input(t_attr!{type="checkbox", name="agree",id="inp--checkbox box1"}),label((t_attr!{for="inp--checkbox"},"By creating an account, you agree to the Terms & Canditions and Privacy Policy in this page")))
                                )),
                                button((t_attr!{type="submit", class="custom-btn btn-13"},"Create Account")),
                                hr(()),
                                a((t_attr!{class="areu",href="#"},"Already have an account?")),
                                p((t_attr!{class="creator"},(span("Made with"),span("❤"),span(" by Sadegh Rastgoo")))),
                                p((t_attr!{class="author"},"案例原链接:https://codier.io/creation/HkYmx8vDu")),
                                p((t_attr!{class="author"},a((t_attr!{href="https://github.com/PrivateRookie/rtml"},"rust crate @rtml:https://github.com/PrivateRookie/rtml"))))
                            )
                        )),
                    )
                )),
            ))
        ),
    ))
}

fn main() {
    println!("{}", index());
}
