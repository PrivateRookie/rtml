use super::TagFormatter;
use crate::Children;
use crate::InnerChildren;
use crate::Kong;
use crate::Tag;
use std::fmt::Write;

macro_rules! impl_basic {
    ($t:ty) => {
        impl Tag for $t {
            fn name(&self) -> &'static str {
                stringify!($t)
            }

            fn format(&self, f: &mut TagFormatter, buf: &mut String) -> std::fmt::Result {
                let pad = f.pad_size();
                write!(buf, "{:pad$}{}{}", "", self, f.line_sep)
            }
        }
    };
}

impl_basic!(String);
impl_basic!(&str);
impl_basic!(bool);
impl_basic!(u8);
impl_basic!(u16);
impl_basic!(u32);
impl_basic!(u64);
impl_basic!(usize);
impl_basic!(i8);
impl_basic!(i16);
impl_basic!(i32);
impl_basic!(i64);
impl_basic!(isize);
impl_basic!(f32);
impl_basic!(f64);

impl std::fmt::Display for Kong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Tag for Kong {
    fn name(&self) -> &'static str {
        ""
    }

    fn format(&self, _f: &mut crate::tags::TagFormatter, _buf: &mut String) -> std::fmt::Result {
        Ok(())
    }
}

impl Tag for () {
    fn name(&self) -> &'static str {
        ""
    }

    fn format(&self, _f: &mut crate::tags::TagFormatter, _buf: &mut String) -> std::fmt::Result {
        Ok(())
    }
}

impl<T> From<Vec<T>> for Children
where
    T: Tag + 'static,
{
    fn from(src: Vec<T>) -> Self {
        let children: InnerChildren = src
            .into_iter()
            .map(|c| {
                let t: Box<dyn Tag> = Box::new(c);
                t
            })
            .collect();
        Self(children)
    }
}

impl<T> From<T> for Children
where
    T: Tag + 'static,
{
    fn from(src: T) -> Self {
        Self(vec![Box::new(src)])
    }
}

impl<const N: usize> From<[Box<dyn Tag>; N]> for Children {
    fn from(src: [Box<dyn Tag>; N]) -> Self {
        let children: InnerChildren = src.into_iter().collect();
        Self(children)
    }
}

macro_rules! tuple_impl {
    ($($t:tt),+ | $($i:tt),+) => {
        impl <$($t ),+ > From<($($t,)+)> for Children
            where
                $($t: Tag + 'static),+
        {
            fn from(src: ($($t,)+)) -> Self {
                let mut children: InnerChildren = Vec::new();
                $(
                    children.push(Box::new(src.$i));
                )+
                Self(children)
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
