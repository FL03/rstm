/*
    Appellation: rstm-core <library>
    Created At: 2026.01.17:21:43:29
    Contrib: @FL03
*/
//! this crate provides core primtives and utilities for working with Turing machines in Rust.
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
    pub(crate) mod rules;
    #[macro_use]
    pub(crate) mod program;
    #[macro_use]
    pub(crate) mod seal;
}
// redeclarations
#[doc(inline)]
pub use rstm_state as state;
// modules
pub mod actors;
pub mod error;
pub mod motion;
pub mod programs;
pub mod rules;

mod utils {
    #[doc(inline)]
    pub use self::range::*;

    mod range;
}

// re-exports (public)
#[doc(inline)]
pub use self::{
    actors::{Driver, Executor, MovingHead},
    error::{Error, Result},
    motion::HeadStep,
    programs::{Program, ProgramBase, RawRuleset},
    rules::prelude::*,
    utils::*,
};
#[cfg(feature = "macros")]
pub use rstm_state::s;
#[doc(inline)]
pub use rstm_state::{Halt, HaltState, Halting, HaltingState, RawState, State, StateExt};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rstm_state::prelude::*;

    #[cfg(all(feature = "alloc", feature = "macros"))]
    pub use crate::program;
    #[cfg(feature = "macros")]
    pub use crate::{rules, ruleset};

    pub use crate::actors::prelude::*;
    pub use crate::motion::prelude::*;
    pub use crate::programs::prelude::*;
    pub use crate::rules::prelude::*;
    pub use crate::utils::*;
}

/// defines the radius around the head position to be displayed when printing the tape
pub const DEFAULT_DISPLAY_RADIUS: usize = 10;
