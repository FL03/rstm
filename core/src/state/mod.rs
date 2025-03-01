/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{halt::*, state::State};

pub(crate) mod state;

pub mod halt;

mod impls {
    mod impl_ops;
    mod impl_repr;
    mod impl_state;
}
pub(crate) mod prelude {
    #[cfg(feature = "std")]
    pub use super::AnyState;
    pub use super::MaybeState;
    pub use super::halt::*;
    pub use super::state::*;
}
#[cfg(feature = "std")]
/// A type alias for a [State] whose inner value is the dynamically sized type of a boxed [`Any`](core::any::Any).
pub type AnyState = State<Box<dyn core::any::Any>>;
/// A type alias for a [State] whose inner value is a [core::mem::MaybeUninit] of generic type `Q`.
pub type MaybeState<Q = bool> = State<core::mem::MaybeUninit<Q>>;

/// [RawState] is a trait describing objects capable of being used as states in our library.
/// The trait contains a single associated trait, the context, or inner value of the state.
#[doc(hidden)]
pub trait RawState {
    type Q;

    private!();

    fn get(self) -> Self::Q;

    fn get_ref(&self) -> &Self::Q;

    fn get_mut(&mut self) -> &mut Self::Q;

    fn set(&mut self, inner: Self::Q);
}

/*
 ************* Implementations *************
*/
macro_rules! impl_raw_state {
    (@impl $state:ident($($field:tt)*)) => {
        impl<Q> RawState for $state<Q> {
            type Q = Q;

            seal!();

            fn get(self) -> Q {
                self.$($field)*
            }

            fn get_ref(&self) -> &Q {
                &self.$($field)*
            }

            fn get_mut(&mut self) -> &mut Q {
                &mut self.$($field)*
            }

            fn set(&mut self, inner: Q) {
                self.$($field)* = inner;
            }
        }
    };
    ($($T:ident($($field:tt)*)),* $(,)?) => {
        $(
            impl_raw_state!(@impl $T($($field)*));
        )*
    };
}

impl_raw_state! {
    Halt(0),
    State(0),
}
