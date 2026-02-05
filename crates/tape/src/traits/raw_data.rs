/*
    Appellation: raw_tape <module>
    Created At: 2026.01.14:20:40:26
    Contrib: @FL03
*/

pub trait RawData {
    type Elem;
}

macro_rules! impl_raw_data {
    (@impl<Elem = $E:ident> $trait:ident for $($cont:ident)::*<$($T:ident),* $(,)?>) => {
        impl<$($T),*> $trait for $($cont)::*<$($T),*> {
            type Elem = $E;
        }
    };
    (impl<Elem = $E:ident> $trait:ident for {$($($cont:ident)::*<$($T:ident),* $(,)?>),* $(,)?}) => {
        $(impl_raw_data! { @impl<Elem = $E> $trait for $($cont)::*<$($T),*> } )*
    };
}

impl_raw_data! {
    impl<Elem = T> RawData for {
        alloc::vec::Vec<T>,
    }
}
