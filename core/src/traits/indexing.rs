/*
    Appellation: indexing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[doc(hidden)]
pub trait RawIndex {
    private!();
}

pub trait Index: RawIndex {
    fn increment(self) -> Self;

    fn decrement(self) -> Self;
}

pub trait HashIndex: Index + core::cmp::Eq + core::hash::Hash {}

/*
 ************* Implementations *************
*/
impl<T> Index for T
where
    T: RawIndex + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + num::One,
{
    fn increment(self) -> Self {
        self + T::one()
    }

    fn decrement(self) -> Self {
        self - T::one()
    }
}

impl<T> HashIndex for T where T: Index + core::cmp::Eq + core::hash::Hash {}

macro_rules! impl_index {
    (@impl $T:ty) => {
        impl RawIndex for $T {
            seal!();
        }
    };
    ($($T:ty),* $(,)?) => {
        $(
            impl_index!(@impl $T);
        )*
    };
}

impl_index!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);
