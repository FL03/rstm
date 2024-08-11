/*
    Appellation: fmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! unit_impl_fmt {
    (@impl $trait:ident::<$T:ty>($($fmt:tt)*)) => {
        impl core::fmt::$trait for $T {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::$trait::fmt(self, f, $($fmt)*)
            }
        }
    };
    ($T:ty: $($trait:ident($($fmt:tt)*)),*) => {
        $(impl_fmt!(@impl $trait::<$T>($($fmt)*));)*
    };
}
