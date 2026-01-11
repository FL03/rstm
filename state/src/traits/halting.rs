/*
    appellation: halted <module>
    authors: @FL03
*/
use crate::state::State;

/// The [`IsHalted`] trait defines a contract for types that can be checked for a halted state.
pub trait IsHalted {
    /// returns true if the current state is considered to be _halted_, otherwise false.
    fn is_halted(&self) -> bool;
}

/*
 ************* Implementations *************
*/
use crate::Halter;

impl<Q, H> IsHalted for State<Halter<Q, H>> {
    fn is_halted(&self) -> bool {
        matches!(self.get(), &Halter::Halt(_))
    }
}
impl<Q> IsHalted for State<Option<Q>> {
    fn is_halted(&self) -> bool {
        self.0.is_none()
    }
}

impl<Q> IsHalted for Option<State<Q>> {
    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

macro_rules! impl_is_halted {
    ($($tag:ident {$($T:ty),* $(,)?}),* $(,)?) => {
        $(impl_is_halted! { @impl #[$tag] $($T),* })*
    };
    (@impl #[unsigned] $($T:ty),*) => {
        $(impl IsHalted for $T {
            fn is_halted(&self) -> bool {
                self == &<$T>::MAX
            }
        })*
    };
    (@impl #[signed] $($T:ty),*) => {
        $(impl IsHalted for $T {
            fn is_halted(&self) -> bool {
                self.abs() == <$T>::MAX
            }
        })*
    };
    (@impl #[float]$($T:ty),*) => {
        $(impl IsHalted for $T {
            fn is_halted(&self) -> bool {
                self.is_nan() || self.is_infinite()
            }
        })*
    };
}

impl_is_halted! {
    unsigned { u8, u16, u32, u64, u128, usize },
    signed { i8, i16, i32, i64, i128, isize },
    float { f32, f64 }
}
