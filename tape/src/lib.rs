/*
    Appellation: rstm-tape <library>
    Created At: 2025.08.30:16:50:58
    Contrib: @FL03
*/
//! Idealized Turing machines consider a tape, or memory, that is infinite in both directions.
//! This tape is a one-dimensional array of symbols manipulated by the tape head according to
//! some set of pre-defined rules.
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::self_named_constructors,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate rstm_core as rstm;

#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
}

#[doc(inline)]
#[cfg(feature = "alloc")]
pub use self::std_tape::StdTape;
#[doc(inline)]
pub use self::{
    error::{Error, Result},
    traits::*,
    types::*,
};

#[doc(inline)]
#[cfg(feature = "std")]
pub use self::hash_tape::HashTape;

pub mod error;

pub mod store;

#[cfg(feature = "std")]
pub mod hash_tape;
#[cfg(feature = "alloc")]
pub mod std_tape;

pub mod traits {
    //! Traits for defining and manipulating Turing machine tapes
    #[doc(inline)]
    pub use self::prelude::*;

    mod fetch;
    mod memory;
    mod tape;

    mod prelude {
        #[doc(inline)]
        pub use super::fetch::*;
        #[doc(inline)]
        pub use super::memory::*;
        #[doc(inline)]
        pub use super::tape::*;
    }
}

pub mod types {
    //! Supporting types for constructing so-called tapes for Turing machines
    #[doc(inline)]
    pub use self::prelude::*;

    mod cell;
    mod snapshot;

    mod prelude {
        #[doc(inline)]
        pub use super::cell::Cell;
        #[doc(inline)]
        pub use super::snapshot::Snapshot;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "std")]
    pub use crate::hash_tape::HashTape;
    #[cfg(feature = "alloc")]
    pub use crate::std_tape::StdTape;
    pub use crate::store::*;
    pub use crate::traits::*;
    pub use crate::types::*;
}
