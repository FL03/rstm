/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! the state module implements the [`State`] type and its associated traits and types
#[doc(inline)]
pub use self::{error::*, halt::*, state::State, traits::*};

mod state;

pub mod error;
pub mod halt;

mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    mod halted;
    mod raw_state;
    mod stated;

    mod prelude {
        #[doc(inline)]
        pub use super::halted::*;
        #[doc(inline)]
        pub use super::raw_state::*;
        #[doc(inline)]
        pub use super::stated::*;
    }
}

pub(crate) mod prelude {
    #[cfg(feature = "alloc")]
    pub use super::AnyState;
    #[doc(inline)]
    pub use super::MaybeState;

    #[doc(inline)]
    pub use super::error::*;
    #[doc(inline)]
    pub use super::halt::*;
    #[doc(inline)]
    pub use super::state::*;
    #[doc(inline)]
    pub use super::traits::*;
}

#[cfg(feature = "alloc")]
/// A type alias for a [State] whose inner value is the dynamically sized type of a boxed [`Any`](core::any::Any).
pub type AnyState = State<alloc::boxed::Box<dyn core::any::Any>>;
/// A type alias for a [State] whose inner value is a [core::mem::MaybeUninit] of generic type `Q`.
pub type MaybeState<Q = bool> = State<core::mem::MaybeUninit<Q>>;
