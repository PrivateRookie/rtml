use std::{
    cell::{Ref, RefCell, RefMut},
    ops::DerefMut,
    rc::Rc,
};

use web_sys::{Element, Event};

use crate::{
    tags::{config_attr, config_style, EleContent, StaticAttrs, StaticStyles},
    StaticContent,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DomPosition {
    pub depth: usize,
    pub index: usize,
}

impl std::fmt::Display for DomPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("dom")
            .field(&self.depth)
            .field(&self.index)
            .finish()
    }
}

impl From<(usize, usize)> for DomPosition {
    fn from((depth, index): (usize, usize)) -> Self {
        Self { depth, index }
    }
}

pub struct Dom {
    path: Vec<usize>,
    children: Vec<Dom>,
    updater: Option<(Element, UpdateFunc)>,
}

#[derive(Clone)]
pub struct UpdateFunc {
    pub attr: Option<Rc<RefCell<dyn Fn() -> StaticAttrs>>>,
    pub style: Option<Rc<RefCell<dyn Fn() -> StaticStyles>>>,
    pub children: Option<Rc<RefCell<dyn Fn() -> StaticContent>>>,
}

impl Dom {
    pub fn pad(path: Vec<usize>) -> Self {
        Self {
            path,
            children: vec![],
            updater: None,
        }
    }

    pub(crate) fn detach_child(&mut self) {
        while let Some(mut child) = self.children.pop() {
            child.detach_child();
            if let Some((ele, _)) = child.updater {
                ele.remove()
            }
        }
    }

    pub(crate) fn register(&mut self, path: Vec<usize>, ele: Element, func: UpdateFunc) {
        let mut path_clone = path.clone();
        path_clone.remove(0);
        path_clone.push(0);
        let mut dom = self;
        let mut dom_path = dom.path.clone();
        for (level, (offset, next)) in path.iter().zip(path_clone.iter()).enumerate() {
            let dom_level = if dom_path.is_empty() {
                0
            } else {
                dom_path.len() - 1
            };
            let dom_offset = dom.path.last().unwrap();
            if (level, *offset) == (dom_level, *dom_offset) && level + 1 == path.len() {
                dom.updater = Some((ele, func));
                break;
            } else {
                if dom.children.get_mut(*next).is_none() {
                    let mut idx = if dom.children.is_empty() {
                        0
                    } else {
                        dom.children.len()
                    };
                    while idx <= *next {
                        let mut path = dom_path.clone();
                        path.push(idx);
                        dom.children.push(Dom::pad(path));
                        idx += 1;
                    }
                }
                dom = dom.children.get_mut(*next).unwrap();
                dom_path = dom.path.clone();
            }
        }
    }
}

pub struct Reactive<T> {
    pub data: Rc<RefCell<T>>,
    pub subs: Rc<RefCell<Dom>>,
}

impl<T> Clone for Reactive<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            subs: self.subs.clone(),
        }
    }
}

pub fn reactive<T>(data: T) -> Reactive<T> {
    Reactive {
        data: Rc::new(RefCell::new(data)),
        subs: Rc::new(RefCell::new(Dom::pad(vec![0]))),
    }
}

pub trait IntoReactive {
    fn reactive(self) -> Reactive<Self>
    where
        Self: Sized;
}

impl<T: Sized> IntoReactive for T {
    fn reactive(self) -> Reactive<Self>
    where
        Self: Sized,
    {
        reactive(self)
    }
}

impl<T: 'static> Reactive<T> {
    /// get inner data ref
    pub fn val(&self) -> Ref<T> {
        self.data.borrow()
    }

    /// get inner data mut ref
    pub fn val_mut(&self) -> RefMut<T> {
        self.data.borrow_mut()
    }

    /// subscribe data change and modify element attributes
    pub fn attr<V: Fn(Self) -> crate::tags::StaticAttrs + 'static>(
        &self,
        v: V,
    ) -> crate::tags::Attrs {
        use crate::tags::Attrs;

        let data = self.clone();
        Attrs::Dynamic {
            subs: vec![self.subs.clone()],
            func: Rc::new(RefCell::new(move || v(data.clone()))),
        }
    }

    /// subscribe data change and modify element styles
    pub fn style<V: Fn(Self) -> crate::tags::StaticStyles + 'static>(
        &self,
        v: V,
    ) -> crate::tags::Styles {
        use crate::tags::Styles;

        let data = self.clone();
        Styles::Dynamic {
            subs: vec![self.subs.clone()],
            func: Rc::new(RefCell::new(move || v(data.clone()))),
        }
    }

    /// subscribe data change and return children
    pub fn view<C: Into<StaticContent>, V: Fn(Self) -> C + 'static>(&self, v: V) -> EleContent {
        let data = self.clone();
        EleContent::Dynamic {
            subs: vec![self.subs.clone()],
            func: Rc::new(RefCell::new(move || v(data.clone()).into())),
        }
    }

    /// handle event and reactive data
    pub fn evt<M: Fn(Event, Self) -> bool + 'static + Clone>(
        &self,
        m: M,
    ) -> Box<dyn Fn() -> Box<dyn Fn(Event)> + 'static> {
        let data = self.clone();
        Box::new(move || {
            let data = data.clone();
            let m = m.clone();
            Box::new(move |event| {
                if m(event, data.clone()) {
                    data.update_and_clear().unwrap();
                }
            })
        })
    }

    /// mutate data and update view
    pub fn change<M: Fn(Self) -> bool + 'static + Clone>(
        &self,
        m: M,
    ) -> Box<dyn Fn() -> Box<dyn Fn(Event)> + 'static> {
        let rea = self.clone();
        Box::new(move || {
            let rea = rea.clone();
            let m = m.clone();
            Box::new(move |_| {
                if m(rea.clone()) {
                    rea.update_and_clear().unwrap();
                }
            })
        })
    }

    pub fn update(&self) {
        self.update_and_clear().unwrap();
    }

    fn update_and_clear(&self) -> Result<(), wasm_bindgen::JsValue> {
        let mut dom = self.subs.borrow_mut();
        let dom = dom.deref_mut();
        update_dom(dom)?;
        Ok(())
    }
}

fn update_dom(dom: &mut Dom) -> Result<(), wasm_bindgen::JsValue> {
    let dom_path = dom.path.clone();
    let mut go_on = true;
    if let Some((ele, updater)) = dom.updater.clone() {
        if let Some(c_func) = updater.children.as_ref() {
            let children = c_func.borrow()();
            match children {
                StaticContent::Null => {
                    ele.set_inner_html("");
                }
                StaticContent::Text(text) => {
                    ele.set_inner_html(&text);
                }
                StaticContent::List(children) => {
                    let window = web_sys::window().expect("no global `window` exists");
                    let document = window.document().expect("should have a document on window");
                    dom.detach_child();
                    while ele.child_element_count() > 0 {
                        if let Some(child) = ele.first_element_child() {
                            child.remove();
                        }
                    }
                    for (idx, child) in children.iter().enumerate() {
                        let mut path = dom_path.clone();
                        path.push(idx);
                        let child = child.render(path, &ele, &document)?;
                        ele.append_child(&child)?;
                    }
                    go_on = false;
                }
            }
        };
        if let Some(a_func) = updater.attr.as_ref() {
            let attrs = a_func.borrow()();
            config_attr(&attrs, &ele)?;
        }
        if let Some(s_func) = updater.style.as_ref() {
            let styles = s_func.borrow()();
            config_style(&styles, &ele)?;
        }
    };

    if go_on {
        for child in dom.children.iter_mut() {
            update_dom(child)?;
        }
    }
    Ok(())
}

pub struct CombinedReactive<T> {
    data: T,
}

impl<T: Clone> Clone for CombinedReactive<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<A, B> std::ops::Add<Reactive<B>> for Reactive<A> {
    type Output = CombinedReactive<(Reactive<A>, Reactive<B>)>;

    fn add(self, rhs: Reactive<B>) -> Self::Output {
        CombinedReactive { data: (self, rhs) }
    }
}

macro_rules! impl_combine {
    ($($t:tt),+ | $($i:tt),+) => {
        impl<This, $($t ),+> std::ops::Add<($($crate::Reactive<$t>),+,)> for $crate::Reactive<This> {
            type Output = $crate::CombinedReactive<($crate::Reactive<This>, $($crate::Reactive<$t>),+)>;

            fn add(self, rhs: ($($crate::Reactive<$t>),+,)) -> Self::Output {
                $crate::CombinedReactive {
                    data: (self.clone(), $(rhs.$i),+)
                }
            }
        }


        impl<$($t: 'static),+> $crate::CombinedReactive<($($crate::Reactive<$t>,)+)> {
            /// subscribe data change
            pub fn attr<Func: Fn(($($crate::Reactive<$t>,)+)) -> $crate::tags::StaticAttrs + 'static>(
                &self,
                func: Func,
            ) -> $crate::tags::Attrs  {
                let data = self.data.clone();
                crate::tags::Attrs::Dynamic {
                    func: ::std::rc::Rc::new(::std::cell::RefCell::new(move || func(data.clone()))),
                    subs: vec![$(self.data.$i.subs.clone()),+]
                }
            }

            /// subscribe data change
            pub fn style<Func: Fn(($($crate::Reactive<$t>,)+)) -> $crate::tags::StaticStyles + 'static>(
                &self,
                func: Func,
            ) -> $crate::tags::Styles  {
                let data = self.data.clone();
                crate::tags::Styles::Dynamic {
                    func: ::std::rc::Rc::new(::std::cell::RefCell::new(move || func(data.clone()))),
                    subs: vec![$(self.data.$i.subs.clone()),+]
                }
            }


            /// subscribe data change
            pub fn view<Content: ::std::convert::Into<$crate::StaticContent> ,View: Fn(($($crate::Reactive<$t>,)+)) -> Content + 'static>(
                &self,
                view: View,
            ) -> $crate::EleContent  {
                let data = self.data.clone();
                let subs = vec![$(self.data.$i.subs.clone()),+];
                $crate::EleContent::Dynamic {
                    func: ::std::rc::Rc::new(::std::cell::RefCell::new(move || view(data.clone()).into())),
                    subs
                }
            }

            /// mutate data and update view
            pub fn change<Method: Fn(($($crate::Reactive<$t>,)+)) -> bool + 'static + Copy>(
                &self,
                m: Method,
            ) -> Box<dyn Fn() -> Box<dyn Fn(web_sys::Event)> + 'static>  {
                let data = self.data.clone();
                Box::new(move|| {
                    let data = data.clone();
                    Box::new(move |_| {
                        if m(data.clone()) {
                            $(
                                tracing::info!("call to update {}", $i);
                                data.$i.update();
                            )+
                        }
                    })
                })
            }

            /// mutate data and update view
            pub fn evt<Method: Fn(web_sys::Event, ($($crate::Reactive<$t>,)+)) -> bool + 'static + Copy>(
                &self,
                m: Method,
            ) -> Box<dyn Fn() -> Box<dyn Fn(web_sys::Event)> + 'static>  {
                let data = self.data.clone();
                Box::new(move|| {
                    let data = data.clone();
                    Box::new(move |event| {
                        if m(event, data.clone()) {
                            $(
                                data.$i.update();
                            )+
                        }
                    })
                })
            }
        }
    };
}

#[rustfmt::skip]
mod inner {
    impl_combine!( A  |  0 );
    impl_combine!( A, B  |  0, 1 );
    impl_combine!( A, B, C  |  0, 1, 2 );
    impl_combine!( A, B, C, D  |  0, 1, 2, 3 );
    impl_combine!( A, B, C, D, E  |  0, 1, 2, 3, 4 );
    impl_combine!( A, B, C, D, E, F  |  0, 1, 2, 3, 4, 5 );
    impl_combine!( A, B, C, D, E, F, G  |  0, 1, 2, 3, 4, 5, 6 );
    impl_combine!( A, B, C, D, E, F, G, H  |  0, 1, 2, 3, 4, 5, 6, 7 );
    impl_combine!( A, B, C, D, E, F, G, H, I  |  0, 1, 2, 3, 4, 5, 6, 7, 8 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24 );
    impl_combine!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 );
}
