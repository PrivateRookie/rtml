use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    rc::Rc,
};

use web_sys::{Element, Event};

use crate::{
    render_children,
    tags::{config_attr, config_style, StaticAttrs, StaticStyles},
    AttrFunc, AttrToken, Children, StyleFunc, StyleToken, ViewSubs, ViewToken,
};

pub struct Reactive<T> {
    pub data: Rc<RefCell<T>>,
    pub view_subs: ViewSubs,
    pub style_subs: Rc<RefCell<Vec<(Element, StyleFunc)>>>,
    pub attr_subs: Rc<RefCell<Vec<(Element, AttrFunc)>>>,
}

impl<T> Clone for Reactive<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            view_subs: self.view_subs.clone(),
            style_subs: self.style_subs.clone(),
            attr_subs: self.attr_subs.clone(),
        }
    }
}

pub fn reactive<T>(data: T) -> Reactive<T> {
    Reactive {
        data: Rc::new(RefCell::new(data)),
        view_subs: Rc::new(RefCell::new(vec![])),
        style_subs: Rc::new(RefCell::new(vec![])),
        attr_subs: Rc::new(RefCell::new(vec![])),
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

    /// subscribe data change and return children
    pub fn attr<V: Fn(Self) -> StaticAttrs + 'static>(&self, v: V) -> AttrToken {
        let data = self.clone();
        AttrToken::Owned {
            subs: self.attr_subs.clone(),
            view: RefCell::new(Box::new(move || v(data.clone()))),
        }
    }

    /// subscribe data change and return children
    pub fn style<V: Fn(Self) -> StaticStyles + 'static>(&self, v: V) -> StyleToken {
        let data = self.clone();
        StyleToken::Owned {
            subs: self.style_subs.clone(),
            view: RefCell::new(Box::new(move || v(data.clone()))),
        }
    }

    /// subscribe data change and return children
    pub fn view<V: Fn(Self) -> Children + 'static>(&self, v: V) -> ViewToken {
        let data = self.clone();
        ViewToken::Owned {
            subs: self.view_subs.clone(),
            view: Cell::new(Box::new(move || v(data.clone()))),
        }
    }

    /// handle event and reactive data
    pub fn evt<M: Fn(Event, Self) + 'static + Copy>(
        &self,
        m: M,
    ) -> Box<dyn Fn() -> Box<dyn Fn(Event)> + 'static> {
        let data = self.clone();
        Box::new(move || {
            let data = data.clone();
            Box::new(move |event| {
                m(event, data.clone());
                update_and_clear(&data);
            })
        })
    }

    /// mutate data and update view
    pub fn change<M: Fn(Self) + 'static + Copy>(
        &self,
        m: M,
    ) -> Box<dyn Fn() -> Box<dyn Fn(Event)> + 'static> {
        let rea = self.clone();
        Box::new(move || {
            let rea = rea.clone();
            Box::new(move |_| {
                m(rea.clone());
                update_and_clear(&rea);
            })
        })
    }

    pub fn update(&self) {
        update_and_clear(self);
    }
}

fn update_and_clear<T>(rea: &Reactive<T>) {
    for (_, ele, view) in rea.view_subs.borrow().iter() {
        let children = ele.children();
        for _ in 0..children.length() {
            if let Some(child) = children.get_with_index(0) {
                ele.remove_child(&child).unwrap();
            }
        }
        let content = view.get_children();
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        if let Err(e) = render_children(&content, ele, &document) {
            tracing::error!("failed to update content: {:?}", e);
        }
    }

    // remove not active elements
    // rea.subscribers
    //     .borrow_mut()
    //     .sort_by(|a, b| (!a.1.is_connected()).cmp(&(!b.1.is_connected())));
    // let pos = rea
    //     .subscribers
    //     .borrow()
    //     .iter()
    //     .position(|i| !i.1.is_connected());
    // if let Some(pos) = pos {
    //     rea.subscribers.borrow_mut().drain(pos..);
    // }

    for (ele, func) in rea.attr_subs.borrow().iter() {
        let attrs = func.val();
        config_attr(&attrs, ele).unwrap();
    }

    for (ele, func) in rea.style_subs.borrow().iter() {
        let styles = func.val();
        config_style(&styles, ele).unwrap();
    }
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


        impl<This: 'static, $($t: 'static),+> $crate::CombinedReactive<($crate::Reactive<This>, $($crate::Reactive<$t>),+)> {
            /// subscribe data change
            pub fn attr<Func: Fn(($crate::Reactive<This>, $($crate::Reactive<$t>),+)) -> $crate::tags::StaticAttrs + 'static>(
                &self,
                func: Func,
            ) -> $crate::AttrToken  {
                let data = self.data.clone();
                $crate::AttrToken::Shared {
                    view: ::std::rc::Rc::new(::std::cell::RefCell::new(move || func(data.clone()))),
                    subs: vec![$(self.data.$i.attr_subs.clone()),+]
                }
            }

            /// subscribe data change
            pub fn style<Func: Fn(($crate::Reactive<This>, $($crate::Reactive<$t>),+)) -> $crate::tags::StaticStyles + 'static>(
                &self,
                func: Func,
            ) -> $crate::StyleToken  {
                let data = self.data.clone();
                $crate::StyleToken::Shared {
                    view: ::std::rc::Rc::new(::std::cell::RefCell::new(move || func(data.clone()))),
                    subs: vec![$(self.data.$i.style_subs.clone()),+]
                }
            }


            /// subscribe data change
            pub fn view<View: Fn(($crate::Reactive<This>, $($crate::Reactive<$t>),+)) -> $crate::Children + 'static>(
                &self,
                view: View,
            ) -> $crate::ViewToken  {
                let data = self.data.clone();
                $crate::ViewToken::Shared {
                    view: ::std::rc::Rc::new(::std::cell::RefCell::new(move || view(data.clone()))),
                    subs: vec![$(self.data.$i.view_subs.clone()),+]
                }
            }

            /// mutate data and update view
            pub fn change<Method: Fn(($crate::Reactive<This>, $($crate::Reactive<$t>),+)) + 'static + Copy>(
                &self,
                m: Method,
            ) -> Box<dyn Fn() -> Box<dyn Fn(web_sys::Event)> + 'static>  {
                let data = self.data.clone();
                Box::new(move|| {
                    let data = data.clone();
                    Box::new(move |_| {
                        m(data.clone());
                        $(
                            data.$i.update();
                        )+
                    })
                })
            }

            /// mutate data and update view
            pub fn evt<Method: Fn(web_sys::Event, ($crate::Reactive<This>, $($crate::Reactive<$t>),+)) + 'static + Copy>(
                &self,
                m: Method,
            ) -> Box<dyn Fn() -> Box<dyn Fn(web_sys::Event)> + 'static>  {
                let data = self.data.clone();
                Box::new(move|| {
                    let data = data.clone();
                    Box::new(move |event| {
                        m(event, data.clone());
                        $(
                            data.$i.update();
                        )+
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
