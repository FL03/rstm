/*
    Appellation: rstm-state <library>
    Created At: 2025.08.29:22:43:22
    Contrib: @FL03
*/
//! The [`state`](self) module provides abstractions and implementations for managing state
//! within the `rstm` framework.
//!
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
    #[macro_use]
    pub(crate) mod state;
}

#[doc(inline)]
pub use self::{error::*, state::State, traits::*, types::*};

mod state;

pub mod error;

mod impls {
    mod impl_state;
    mod impl_state_ops;
    mod impl_state_repr;

    #[allow(deprecated)]
    mod impl_state_deprecated;
}

pub mod traits {
    //! the traits for defining the types of states and their behaviors
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

pub mod types {
    //! various types and type aliases for working with state
    #[doc(inline)]
    pub use self::prelude::*;

    mod halt;

    mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::halt::*;
    }

    mod aliases {
        use crate::state::State;
        #[cfg(feature = "alloc")]
        /// A type alias for a [State] whose inner value is the dynamically sized type of a
        /// boxed [`Any`](core::any::Any).
        pub type AnyState = State<alloc::boxed::Box<dyn core::any::Any>>;
        /// A type alias for a [State] whose inner value is a [core::mem::MaybeUninit] of
        /// generic type `Q`.
        pub type MaybeState<Q = bool> = State<core::mem::MaybeUninit<Q>>;
        #[cfg(feature = "alloc")]
        pub type SharedState<Q> = State<alloc::sync::Arc<Q>>;
        #[cfg(feature = "std")]
        pub type ShardState<Q> = State<std::sync::Arc<std::sync::Mutex<Q>>>;
        /// A type alias for a [State] whose inner value is a reference to a generic type `Q`.
        pub type RefState<'a, Q> = State<&'a Q>;
        /// A type alias for a [State] whose inner value is a mutable reference to a generic
        ///  type `Q`.
        pub type MutState<'a, Q> = State<&'a mut Q>;
        /// A type alias for a [State] whose inner value is a raw pointer to a generic type `Q`
        pub type PtrState<Q> = State<*mut Q>;
    }
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::state;
    #[doc(no_inline)]
    pub use crate::state::*;
    #[doc(no_inline)]
    pub use crate::traits::*;
    #[doc(no_inline)]
    pub use crate::types::*;
}
