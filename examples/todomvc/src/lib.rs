
// use std::ops::Add;
use std::{cell::RefCell, rc::Rc};

// use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlInputElement};

use rtml::{attr, mount_body, style, tags::*};
use rtml::EventKind::{Change, Click};

// - https://rustwasm.github.io/docs/wasm-bindgen/
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace= console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace= console)]
    fn info(s: bool, idx: usize);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Clone)]
struct Item<'a> {
    id: usize,
    msg: String,
    item_active_or_inactive_color: &'a str,
    done: bool,
}

impl Item<'_> {
    fn new(id: usize, msg: String) -> Self {
        Self {
            id,
            msg,
            done: false,
            item_active_or_inactive_color: "black",  //可删除，颜色状态交给StatusStore管理，根据done值更改并更新
        }
    }
    fn toggle(&mut self) {
        self.done = !self.done;
    }
}

#[derive(Debug,Copy, Clone)]
enum IsShow {
    SHOW,
    HIDE,
}

#[derive(Debug, Clone)]
struct ActiveStatusData<'a>(Vec<Item<'a>>);

// #[derive(Debug)]
// todo_mvc app status
struct StatusStore<'a> {
    task_lists: Reactive<Vec<Item<'a>>>,
    is_show: Reactive<IsShow>,
    message: Reactive<String>,
    // active_or_inactive_color: Reactive<&'a str>,
    task_lists_len: Reactive<usize>,
    active_status_data: Reactive<ActiveStatusData<'a>>
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    let status_store = StatusStore {
        task_lists: vec![Item::new(0, "默认活动".to_string())].reactive(),
        is_show: IsShow::SHOW.reactive(),
        message: String::new().reactive(),
        // active_or_inactive_color: "black".reactive(),
        task_lists_len: 1.reactive(),
        active_status_data:ActiveStatusData(vec![Item::new(0, "默认活动".to_string())]).reactive()
    }
    .reactive();

    // 控制台 print Reactive 类型的数据
    let test: Reactive<&str> = "hello ,我是正在学编程的小伙伴.前端 and Rust".into();
    log(*test.val());

    let is_show;
    let show_card_enum = status_store.val().is_show.clone();
    // log(&format!("{}", show_card_enum));
    // span(format!("{} items left", store.val().task_lists_len.val())).into()
    match *show_card_enum.val() {
        IsShow::SHOW => is_show = "show-card",
        IsShow::HIDE => is_show = "hide-card",
    }

    log(&format!("is {}",is_show));

    // header component
    let new_todo_change_listener = status_store.evt(|event, status_store| {
        let evt: Event = event.into();
        if let Some(target) = evt.target() {
            // let value = Reflect::get(&target, &JsValue::from_str("value")).unwrap();
            let input: HtmlInputElement = JsValue::from(target).into();
            let value = input.value();
            // if let Some(val) = value.as_string() {
            //     *input_msg.val_mut() = val;
            // }
            status_store.val_mut().message = value.reactive();
            // status_store.update();
            input.set_value("");
        }
    });

    let new_todo_click_listener = status_store.change(|store| {
        // let _store:Rc<RefCell<StatusStore>> = store.into();
        // let msg:Rc<RefCell<String>> = (*_store.borrow()).message.clone().into();
        // let message = (msg.borrow()).clone();
        // log(message.as_str());
        // if  !message.is_empty() {
        //     let text = message.clone();
        //     _store.borrow_mut().task_lists.val_mut().push(Item::new(1000, text));
        // _store.borrow_mut().message = String::new().reactive();
        // }
        if !store.val().message.val().is_empty() {
            let text = store.val().message.clone();
            let msg: Rc<RefCell<String>> = text.into();
            let msg = msg.borrow().clone();
            if !msg.is_empty() {
                store
                    .val()
                    .task_lists
                    .val_mut()
                    .push(Item::new(*store.val().task_lists_len.val() + 1, msg));
                *store.val().task_lists_len.val_mut() += 1;
                *store.val().active_status_data.val_mut() = ActiveStatusData(store.val().task_lists.val().to_vec());
                *store.val().message.val_mut() = "".to_string();

                *store.val().is_show.val_mut() = IsShow::SHOW;
                log(&format!("show . {}", store.val().is_show));

                store.update();
            }
            let s = format!("{}", store.val().task_lists);
            // log(&format!("task_lists_len: {}", *store.val().task_lists_len.val()));
            console_log!("task_lists {}", s);
        }
    });

    let header = header((
        h2("todos"),
        div((
            input(
                attr! {type="text", id="new-todo",class="box",placeholder="What needs to be done?"},
            )
            .on(Change, new_todo_change_listener),
            button("+").on(Click, new_todo_click_listener),
        )),
    ));

    // 渲染ul列表  ul component
    let ul_li = ul(status_store.view(|store| {
        store.val().active_status_data.val().0
            .iter()
            .enumerate()
            .map(|(idx, data)| {
                li((
                    attr! {class="box list"},
                    (
                        p((
                            style! {display:"inline-block";width:"20px";height:"20px";color:format!("{}",data.item_active_or_inactive_color)},
                            i(attr! {class="bi bi-file-check-fill"}),
                        ))
                        .on(
                            Click,
                            store.change(move |store| {
                                if let Some(item) = store.val().active_status_data.val_mut().0.get_mut(idx) {
                                    item.toggle();
                                    store.val().active_status_data.update(); 
                                    if item.done {
                                        item.item_active_or_inactive_color = "skyblue";
                                        
                                    }else{
                                        item.item_active_or_inactive_color = "black";
                                    }
                                    // store.task_lists update
                                    if let Some(item) = store.val().task_lists.val_mut().get_mut(idx) {
                                        item.toggle();
                                        store.val().task_lists.update();
                                        if item.done {
                                            item.item_active_or_inactive_color = "skyblue";
                                            
                                        }else{
                                            item.item_active_or_inactive_color = "black";
                                        }
                                    }
                                    info(item.done,idx);
                                };
                                // store.val_mut().active_status_data = ActiveStatusData(store.val().task_lists.clone().val().to_vec()).reactive();
                                // store.val().active_status_data.update(); 
                                // *store.val().active_status_data.val_mut() = ActiveStatusData(store.val().task_lists.val().to_vec());
                                
                            }),
                        ),
                        span((style! {display:"inline-block";}, format!("{}", data.msg))),
                        button("X").on(
                            Click,
                            store.change(move |store| {
                                store.val().active_status_data.val_mut().0.remove(idx);
                                store.val().task_lists.val_mut().remove(idx);
                                *store.val().task_lists_len.val_mut() -= 1;
                                console_log!("{} delete",idx);

                                if store.val().task_lists.val().len() == 0 {
                                    log("0");
                                    *store.val().is_show.val_mut() = IsShow::HIDE;
                                    store.val().is_show.update();

                                    log(&format!("show . {}", store.val().is_show));
                                }
                                store.update();
                            }),
                        ),
                    ),
                ))
            })
            .collect::<Vec<_>>()
            .into()
    }));

    // section component
    let section = section(div((
        attr! {id="todo"},
        (
            ul_li,
            div((
                attr! {class=format!("box card {}",is_show),id="card"},
                (
                    p(status_store.view(|store| {
                        span(format!("{} items left", store.val().task_lists_len.val())).into()
                    })),
                    label((attr! {for="all",class="show-border"}, "All")),
                    input(attr! {type="radio",id="all",name="btn",checked="true"}).on(
                        Click,
                        status_store.evt(|event, store| {
                            let evt: Event = event.into();
                            if let Some(target) = evt.target() {
                                let input_element: HtmlInputElement = JsValue::from(target).into();
                                let checked = input_element.checked();
                                let list_map = store.val().task_lists.clone();
                                if checked {
                                    store.val_mut().active_status_data = ActiveStatusData(list_map.val().to_vec()).reactive();
                                    store.update();
                                    console_log!("{}", "all");
                                    let s = format!("{} ActiveStatusData", store.val().active_status_data);
                                    console_log!("{}",s);
                                }
                            }
                        }),
                    ),
                    label((attr! {for="active"}, "Active")),
                    input(attr! {type="radio",id="active",name="btn"}).on(
                        Click,
                        status_store.evt(|event, store| {
                            let evt: Event = event.into();
                            if let Some(target) = evt.target() {
                                let input_element: HtmlInputElement = JsValue::from(target).into();
                                let checked = input_element.checked();

                                let task_lists = store.val().task_lists.clone();
                                let list = task_lists
                                    .val()
                                    .clone()
                                    .into_iter()
                                    .filter(|item| !item.done)
                                    .collect::<Vec<_>>();
                                if checked {
                                    store.val_mut().active_status_data = ActiveStatusData(list).reactive();
                                    store.update();
                                    console_log!("{}", "active");
                                }
                            }
                        }),
                    ),
                    label((attr! {for="completed"}, "Completed")),
                    input(attr! {type="radio",id="completed",name="btn"}).on(Click,status_store.evt(|event,store| {
                        let evt: Event = event.into();
                        if let Some(target) = evt.target() {
                            let input_element:HtmlInputElement = JsValue::from(target).into();
                            let checked = input_element.checked();
                            let task_lists = store.val().task_lists.clone();
                            let list = task_lists
                            .val()
                            .clone()
                            .into_iter()
                            .filter(|item| item.done)
                            .collect::<Vec<_>>();
                            if checked {
                                store.val_mut().active_status_data = ActiveStatusData(list).reactive();
                                store.update();
                                console_log!("{}", "completed");
                            }
                        }
                    })),
                ),
            )),
        ),
    )));

    // footer component
    let footer = footer(div(
        a((attr!{href="rust crate @rtml:https://github.com/PrivateRookie/rtml",target="_blank"},"rust crate @rtml:https://github.com/PrivateRookie/rtml"))
    ));
    mount_body(div((attr! {id="todo-app"}, (header, section, footer)))).unwrap();
}

//function component
// fn section_component<T: Into<Option<&'static str>>, V: Into<Option<usize>>>(
//     is_show: T,
//     items_len: V,
//     ul: rtml::tags::Ul,
// ) -> rtml::tags::Section {
//     let div = div((attr! {id="todo"}, (ul)));
//     section(div)
// }

// for list in lists.val().iter() {
//     li_html.push(format!("<li class=\"box list\"><p style=\"display:\"inline-block\";width:\"20px\";height:\"20px\"\"><i class=\" bi bi-file-check-fill\"></i></p><span>s-{}</span><button>X</button></li>",list.msg));
// }
