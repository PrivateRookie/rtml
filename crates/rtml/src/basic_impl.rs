use crate::{Marker, Markers, Merge};

impl<D> Markers for Marker<D> {
    fn set_this(&self, element: web_sys::Element) {
        self.set(element);
    }
}

macro_rules! impl_markers {
    ($($ty:tt),+) => {
        impl<$($ty),+> $crate::Markers  for ($($crate::Marker<$ty>),+ ,) {
            fn set_this(&self, element: web_sys::Element) {
                self.0.set(element);
            }
        }
    };
}

impl_markers!(A);
impl_markers!(A, B);
impl_markers!(A, B, C);
impl_markers!(A, B, C, D);
impl_markers!(A, B, C, D, E);
impl_markers!(A, B, C, D, E, F);
impl_markers!(A, B, C, D, E, F, G);
impl_markers!(A, B, C, D, E, F, G, H);
impl_markers!(A, B, C, D, E, F, G, H, I);
impl_markers!(A, B, C, D, E, F, G, H, I, J);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
impl_markers!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA);

impl<Init: Clone, A: Clone> Merge<Marker<A>> for Marker<Init> {
    type Output = (Marker<Init>, Marker<A>);

    fn merge(self, rhs: Marker<A>) -> Self::Output {
        (self, rhs)
    }
}

#[rustfmt::skip]
mod merge_impl {
    macro_rules! impl_merge {
        ($s:tt | $($ty:tt),+ | $($i:tt),+) => {
            impl <$s: Clone, $($ty: Clone),+> $crate::Merge<($($crate::Marker<$ty>),+ ,)> for $crate::Marker<$s> {
                type Output = ($crate::Marker<$s>, $($crate::Marker<$ty>),+);
    
                fn merge(self, rhs: ($($crate::Marker<$ty>),+ ,)) -> Self::Output {
                    (self, $(rhs.$i),+)
                }
            }
        }
    }

    impl_merge!(Init | A | 0);
    impl_merge!(Init | A, B | 0, 1);
    impl_merge!(Init | A, B, C | 0, 1, 2);
    impl_merge!(Init | A, B, C, D | 0, 1, 2, 3);
    impl_merge!(Init | A, B, C, D, E | 0, 1, 2, 3, 4);
    impl_merge!(Init | A, B, C, D, E, F | 0, 1, 2, 3, 4, 5);
    impl_merge!(Init | A, B, C, D, E, F, G | 0, 1, 2, 3, 4, 5, 6);
    impl_merge!(Init | A, B, C, D, E, F, G, H | 0, 1, 2, 3, 4, 5, 6, 7);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I | 0, 1, 2, 3, 4, 5, 6, 7, 8);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24);
    impl_merge!(Init | A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);
}

mod impl_children {
    use crate::{tags::Content, Template};
    impl Content {}

    impl<T> From<Vec<T>> for Content
    where
        T: Template + 'static,
    {
        fn from(src: Vec<T>) -> Self {
            Self::List(
                src.into_iter()
                    .map(|item| Box::new(item) as Box<dyn Template>)
                    .collect(),
            )
        }
    }

    impl<T> From<T> for Content
    where
        T: Template + 'static,
    {
        fn from(src: T) -> Self {
            Self::List(vec![Box::new(src)])
        }
    }

    impl<const N: usize> From<[Box<dyn Template>; N]> for Content {
        fn from(src: [Box<dyn Template>; N]) -> Self {
            Self::List(src.into_iter().collect())
        }
    }

    impl From<()> for Content {
        fn from(_: ()) -> Self {
            Self::Null
        }
    }

    macro_rules! prime_impl {
        ($ty:ty) => {
            impl From<$ty> for Content {
                fn from(src: $ty) -> Self {
                    Self::Text(src.to_string())
                }
            }
        };
    }

    prime_impl!(String);
    prime_impl!(&String);
    prime_impl!(&str);
    prime_impl!(bool);
    prime_impl!(u8);
    prime_impl!(u16);
    prime_impl!(u32);
    prime_impl!(u64);
    prime_impl!(usize);
    prime_impl!(i8);
    prime_impl!(i16);
    prime_impl!(i32);
    prime_impl!(i64);
    prime_impl!(isize);
    prime_impl!(f32);
    prime_impl!(f64);

    macro_rules! tuple_impl {
        ($($t:tt),+ | $($i:tt),+) => {
            impl <$($t ),+ > From<($($t,)+)> for Content
                where
                    $($t: Template + 'static),+
            {
                fn from(src: ($($t,)+)) -> Self {
                    Self::List(vec![$(Box::new(src.$i) as Box<dyn Template>),+])
                }
            }
        };
    }

    #[rustfmt::skip]
    mod inner {
    use super::*;
    tuple_impl!( A  |  0 );
    tuple_impl!( A, B  |  0, 1 );
    tuple_impl!( A, B, C  |  0, 1, 2 );
    tuple_impl!( A, B, C, D  |  0, 1, 2, 3 );
    tuple_impl!( A, B, C, D, E  |  0, 1, 2, 3, 4 );
    tuple_impl!( A, B, C, D, E, F  |  0, 1, 2, 3, 4, 5 );
    tuple_impl!( A, B, C, D, E, F, G  |  0, 1, 2, 3, 4, 5, 6 );
    tuple_impl!( A, B, C, D, E, F, G, H  |  0, 1, 2, 3, 4, 5, 6, 7 );
    tuple_impl!( A, B, C, D, E, F, G, H, I  |  0, 1, 2, 3, 4, 5, 6, 7, 8 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32 );
    tuple_impl!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD, AE, AF, AG, AH  |  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33 );
    }
}
