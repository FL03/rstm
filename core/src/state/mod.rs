/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{halt::*, state::State, states::*};

pub(crate) mod state;

pub mod halt;

pub(crate) mod states {
    #[doc(inline)]
    pub use self::binary::*;

    pub(crate) mod binary;
}

pub(crate) mod prelude {
    pub use super::halt::*;
    pub use super::state::State;
    pub use super::states::*;
    pub use super::AnyState;
}

pub type AnyState = State<Box<dyn std::any::Any>>;

/// [RawState] is a trait describing objects capable of being used as states in our library.
/// The trait contains a single associated trait, the context, or inner value of the state.
#[doc(hidden)]
pub trait RawState {
    type Inner;

    private!();

    fn into_inner(self) -> Self::Inner;

    fn get(&self) -> &Self::Inner;

    fn get_mut(&mut self) -> &mut Self::Inner;

    fn set(&mut self, inner: Self::Inner);
}

#[doc(hidden)]
pub trait Stateful<Q> {
    type State: RawState<Inner = Q>;
}

pub trait StatefulView<Q>: Stateful<Q> {
    fn state(&self) -> &Self::State;
}

/*
 ************* Implementations *************
*/
macro_rules! impl_raw_state {
    (@impl $T:ident($($field:tt)*)) => {
        impl<Q> RawState for $T<Q> {
            type Inner = Q;

            seal!();

            fn into_inner(self) -> Self::Inner {
                self.$($field)*
            }

            fn get(&self) -> &Self::Inner {
                &self.$($field)*
            }

            fn get_mut(&mut self) -> &mut Self::Inner {
                &mut self.$($field)*
            }

            fn set(&mut self, inner: Self::Inner) {
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

impl<Q> Stateful<Q> for Halt<Q> {
    type State = State<Q>;
}

impl<Q> Stateful<Q> for State<Q> {
    type State = State<Q>;
}
