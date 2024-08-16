/*
    Appellation: fmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! unit_impl_fmt {
    ($trait:ident::<$T:ty>($fmt:expr)) => {
        impl core::fmt::$trait for $T {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                $fmt(&self, f)
            }
        }
    };
}
