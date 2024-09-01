/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{halt::*, state::State, states::*};

pub(crate) mod state;

pub mod halt;

mod impls {
    pub mod impl_ext;
    pub mod impl_ops;
    pub mod impl_repr;
}

pub(crate) mod states {
    #[doc(inline)]
    pub use self::binary::*;

    pub(crate) mod binary;
}

pub(crate) mod prelude {
    pub use super::halt::Haltable;
    pub use super::state::State;
    pub use super::states::*;
    pub use super::AnyState;
}
/// A type alias for a [State] whose inner value is the dynamically sized type of a boxed [`Any`](core::any::Any).
pub type AnyState = State<Box<dyn std::any::Any>>;

/// [RawState] is a trait describing objects capable of being used as states in our library.
/// The trait contains a single associated trait, the context, or inner value of the state.
#[doc(hidden)]
pub trait RawState {
    type Q;

    private!();

    fn into_inner(self) -> Self::Q;

    fn get(&self) -> &Self::Q;

    fn get_mut(&mut self) -> &mut Self::Q;

    fn set(&mut self, inner: Self::Q);
}

#[doc(hidden)]
pub trait Stateful<Q> {
    type State: RawState<Q = Q>;

    fn get(self) -> Q;

    fn get_mut(&mut self) -> &mut Q;

    fn set(&mut self, state: Q);

    fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(Q) -> U,
        Self: Sized;
}

/*
 ************* Implementations *************
*/
macro_rules! impl_raw_state {
    (@impl $state:ident($($field:tt)*)) => {
        impl<Q> RawState for $state<Q> {
            type Q = Q;

            seal!();

            fn into_inner(self) -> Q {
                self.$($field)*
            }

            fn get(&self) -> &Q {
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
