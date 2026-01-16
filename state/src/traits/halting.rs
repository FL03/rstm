/*
    appellation: halted <module>
    authors: @FL03
*/
use crate::state::State;

/// The [`Halting`] trait establishes an interface for determining whether a given state
/// is in a halted condition. This trait is essential for Turing machine simulations,
/// as it allows for the identification of states that signify the end of computation.
pub trait Halting {
    /// returns true if the current state is considered to be _halted_, otherwise false.
    fn is_halted(&self) -> bool;
}

/*
 ************* Implementations *************
*/
use crate::Halt;

impl<Q> Halting for State<Q>
where
    Q: Halting,
{
    fn is_halted(&self) -> bool {
        self.get().is_halted()
    }
}

impl<Q, H> Halting for Halt<Q, H> {
    fn is_halted(&self) -> bool {
        matches!(self, &Halt::Halt(_))
    }
}

impl<Q> Halting for Option<Q> {
    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

macro_rules! impl_is_halted {
    ($($tag:ident {$($T:ty),* $(,)?}),* $(,)?) => {
        $(impl_is_halted! { @impl #[$tag] Halting for $($T),* })*
    };
    (@impl #[unsigned] $trait:ident for $($T:ty),*) => {
        $(impl $trait for $T {
            fn is_halted(&self) -> bool {
                self == &<$T>::MAX
            }
        })*
    };
    (@impl #[signed] $trait:ident for $($T:ty),*) => {
        $(impl $trait for $T {
            fn is_halted(&self) -> bool {
                self.abs() == <$T>::MAX
            }
        })*
    };
    (@impl #[float] $trait:ident for $($T:ty),*) => {
        $(impl $trait for $T {
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
