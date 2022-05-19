use rtml::attr;
use rtml::tags::*;
#[rtml::page]
fn index() -> Html {
    html((
        attr! {lang="en"},
        (
            head((
                meta(attr! { charset="utf-8" }),
                meta(attr!{ http-equiv="X-UA-Compatible", content="IE=edge"}),
                meta(attr! { name="viewport", content="width=device-width, initial-scale=1.0, maximum-scale=1.0"}),
                meta(attr! { name="author", content="Sadegh Rastgoo"}),
                title("LOGIN page-Sadegh Rastgoo"),
                link(attr! {
                    rel="stylesheet",
                    href ="https://www.jsdelivr.com/package/npm/normalize.css"
                }),
                link(attr! { rel="preconnect", href="https://fonts.gstatic.com"}),
                link(attr! { rel="stylesheet", href="https://fonts.googleapis.com/css2?family=Nunito:wght@200;400;700&display=swap"}),
                link(attr! { rel="stylesheet", href="https://cdn.bootcdn.net/ajax/libs/font-awesome/6.1.1/css/fontawesome.min.css"}),
                style(include_str!("../assets/index.css"))
            )),
            body((
                div((
                    attr! {class="main"},
                    (
                        div((
                            attr!{class="card_image-container"},
                            div((
                                attr!{class="slide-images"},
                                (
                                    img(attr!{class="slide",src="https://i.pinimg.com/564x/54/bb/96/54bb962a32c1093f999cb45d89c9dc0e.jpg",alt="Card Image"}),
                                    img(attr!{class="slide",src="https://i.pinimg.com/564x/14/13/38/1413387c9de2609825b61f0719f024d4.jpg",alt="Card Image"}),
                                    img(attr!{class="slide",src="https://i.pinimg.com/564x/6a/46/40/6a4640fd2df70f1e76f388175ffb0349.jpg",alt="Card Image"})
                                )
                            ))
                        )),
                        form((
                            attr! {action="#",autocomplete="off"},
                            (
                                h2((attr!{class="heading-text"},"Sign Up")),
                                div((attr!{class="inp-1"},(i(attr!{class="fa-thin fa-user"}),input(attr!{type="text", name="username",id="inp--username",placeholder="user name"})))),
                                div((attr!{class="inp-1"},(i(attr!{class="fa-thin fa-envelope"}),input(attr!{type="email", name="email",id="inp--email",placeholder="email"})))),
                                div((attr!{class="inp-1"},(i(attr!{class="fa-thin fa-unlock-keyhole"}),input(attr!{type="password", name="password",id="inp--password",placeholder="password"})))),
                                div((
                                    attr!{class="inp-2"},
                                    (input(attr!{type="checkbox", name="agree",id="inp--checkbox box1"}),label((attr!{for="inp--checkbox"},"By creating an account, you agree to the Terms & Canditions and Privacy Policy in this page")))
                                )),
                                button((attr!{type="submit", class="custom-btn btn-13"},"Create Account")),
                                hr(()),
                                a((attr!{class="areu",href="#"},"Already have an account?")),
                                p((attr!{class="creator"},(span("Made with"),span("❤"),span(" by Sadegh Rastgoo")))),
                                p((attr!{class="author"},"案例原链接:https://codier.io/creation/HkYmx8vDu")),
                                p((attr!{class="author"},a((attr!{href="https://github.com/PrivateRookie/rtml"},"rust crate @rtml:https://github.com/PrivateRookie/rtml"))))
                            )
                        )),
                    )
                )),
            ))
        ),
    ))
}
