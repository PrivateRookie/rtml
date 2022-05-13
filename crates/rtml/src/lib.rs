use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element};

mod basic_impl;
mod events;
pub mod tags;
pub use events::*;
use tags::{Attrs, Content, Styles};
mod attr;
mod style;

pub trait Template {
    /// provide tag name, content, attr, styles and listeners
    /// to create html element
    fn resources(
        &self,
    ) -> (
        &'static str,
        &Attrs,
        &Styles,
        &Content,
        HashMap<&str, Box<dyn FnOnce() -> Box<dyn FnMut()> + '_>>,
    );

    /// after calling dom api to create element,
    /// replace marker ele field with real element
    fn set_element(&self, element: Element);

    /// generate html element and add event bindings
    fn render(&self, parent: &Element, doc: &Document) -> Result<Element, JsValue> {
        let (name, attrs, styles, content, listeners) = self.resources();
        let ele = doc.create_element(name).map_err(|e| {
            tracing::error!("failed to create element {}: {:?}", name, e);
            e
        })?;
        for (name, value) in attrs.0.iter() {
            ele.set_attribute(name, value).map_err(|e| {
                tracing::error!("failed to set attribute {}=\"{}\", {:?}", name, value, e);
                e
            })?;
        }

        if !styles.0.is_empty() {
            let styles = styles
                .0
                .iter()
                .map(|(k, v)| format!("{k}:{v};"))
                .collect::<Vec<_>>()
                .join("");
            ele.set_attribute("style", &styles).map(|e| {
                tracing::error!("failed to set style on {}: {:?}", name, e);
            })?;
        }

        for (kind, factory) in listeners.into_iter() {
            let cb = Closure::wrap(factory());
            ele.add_event_listener_with_callback(kind, cb.as_ref().unchecked_ref())
                .map_err(|e| {
                    tracing::error!(
                        "failed to bind {} event listener on {}: {:?}",
                        kind,
                        name,
                        e
                    );
                    e
                })?;
            cb.forget()
        }

        self.set_element(ele.clone());
        parent.append_child(&ele).map_err(|e| {
            tracing::error!(
                "failed to append children {} to parent {}: {:?}",
                name,
                parent.local_name(),
                e
            );
            e
        })?;
        match content {
            Content::Null => {}
            Content::Text(text) => {
                ele.set_inner_html(&text);
            }
            Content::List(children) => {
                for child in children {
                    let child = child.render(&ele, doc)?;
                    ele.append_child(&child)?;
                }
            }
        };
        Ok(ele)
    }
}

pub trait Markers {
    fn set_this(&self, element: Element);
}

#[derive(Debug, Clone)]
pub struct Marker<D = ()> {
    pub ele: Rc<RefCell<Option<Element>>>,
    pub data: Rc<RefCell<D>>,
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            ele: Default::default(),
            data: Rc::new(RefCell::new(())),
        }
    }
}

impl<D> Marker<D> {
    pub fn is_init(&self) -> bool {
        self.ele.borrow().is_some()
    }

    pub fn set(&self, ele: Element) -> Option<Element> {
        self.ele.borrow_mut().replace(ele)
    }

    pub fn to<T>(&self, data: Rc<RefCell<T>>) -> Marker<T> {
        let ele = self.ele.clone();
        Marker { ele, data }
    }
}

pub fn tag(name: &'static str, content: Content) -> tags::Unit<(Marker,)> {
    tags::Unit {
        name,
        content,
        styles: Default::default(),
        attrs: Default::default(),
        markers: (Marker::default(),),
        listeners: Default::default(),
    }
}

pub trait Merge<Rhs = Self> {
    type Output: Markers;

    fn merge(self, rhs: Rhs) -> Self::Output;
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let all_cards = crate::tags::div((
        // 调用其他组件
        count_card("card1".to_string(), Some(100), None),
        count_card("card2".to_string(), Some(20), None),
    ));
    // 渲染
    if let Err(e) = all_cards.render(&body, &document) {
        tracing::error!("failed to init page, {:?}", e);
    }
}

// 某个块可以方便地抽成组件, 组件初始化参数即为函数参数
fn count_card(
    desc: String,
    init: Option<usize>,
    btn_label: Option<String>,
) -> crate::tags::Div<(Marker,)> {
    use crate::tags::*;

    let init = init.unwrap_or_default();
    // 根据初始值设置显示, 注意只是设置 <p> 内容, 不会将 init 和 <p> 绑定
    let show = p(init);
    let incr = button(btn_label.unwrap_or_else(|| "click".to_string()))
        // 将 init 和 <button>绑定
        .take(init)
        // 将 <button> 和 <p> 连接到一起, 方便以后的事件处理
        .link(show.mark())
        // 添加点击事件处理函数
        // 传入的匿名函数第一个参数总是指向正在创建的 ele,
        // 根据 之前 link 参数不同, 其他参数个数也会相应变化
        .on(EventKind::Click, |(btn, show)| {
            Box::new(move || {
                tracing::info!("clicked");
                // 从 btn 拿到从 take 函数中传入的数据
                let mut count = btn.data.borrow_mut();
                // 更新数据
                *count += 1;
                // 从 show 拿到 button 对应的 html 元素, 并进行更新操作
                // rtml 还没做好响应式设计
                let e = show.ele.borrow();
                let e = e.as_ref().unwrap();
                e.set_inner_html(&count.to_string());
            })
        });

    div((
        attr! {
            class="count-card"
        },
        style! {
            border: "black solid 1px";
            padding: "10px";
            margin: "10px 0 10px 0"
        },
        (h2(desc), show, incr),
    ))
}
