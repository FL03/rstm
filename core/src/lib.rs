/*
    Appellation: eryon-rules <library>
    Created At: 2025.12.15:16:51:44
    Contrib: @FL03
*/
//! Rules and their components
//!
//!
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::non_canonical_partial_ord_impl,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// compile error if neither `alloc` nor `std` features are enabled
#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! { "Either the `std` or `alloc` feature must be enabled to compile this crate." }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    #[cfg(feature = "macros")]
    pub mod rules;

    #[macro_use]
    pub mod seal;
}
// redeclarations
#[doc(inline)]
pub use rstm_state as state;
// modules
pub mod actors;
pub mod error;
pub mod rule;

mod cmp {
    #[doc(inline)]
    pub use self::{head::*, tail::*};

    pub mod head;
    pub mod tail;
}

mod traits {
    #[doc(inline)]
    pub use self::{convert::*, rulespace::*};

    mod convert;
    mod rulespace;
}

mod types {
    #[doc(inline)]
    pub use self::direction::*;

    mod direction;
}

// re-exports
#[doc(inline)]
pub use self::{
    actors::MovingHead,
    cmp::*,
    error::{Error, Result},
    rule::*,
    traits::*,
    types::*,
};
#[doc(inline)]
pub use rstm_state::{Halt, HaltState, Haltable, RawState, State};
// prelude
#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "macros")]
    pub use crate::{rule, ruleset, state};

    pub use crate::actors::prelude::*;
    pub use crate::cmp::*;
    pub use crate::rule::*;
    pub use crate::state::*;
    pub use crate::types::*;
}
