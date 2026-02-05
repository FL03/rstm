/*
    Appellation: rstm-core <library>
    Created At: 2026.01.17:21:43:29
    Contrib: @FL03
*/
//! This crate provides the core abstractions and utilities for the `rstm` framework, including
//! common types, traits, and macros that are used across the various components of the system.
//!
//! ## Features
//!
//! - `std`: Enables features that require the Rust standard library.
//! - `alloc`: Enables features that require the Rust `alloc` crate, allowing for dynamic
//!   memory allocation in `no_std` environments.
//! - `nightly`: Enables features that require a nightly Rust compiler, such as certain unstable
//!   features or APIs.
//!
//! ## Overview
//!
//! ### Rules
//!
//! - [`Head`]: A structural implementation of the 2-tuple containing the current
//!   state and symbol of the machine.
//!
//!
#![allow(
    clippy::len_without_is_empty,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::new_ret_no_self,
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
#[doc(inline)]
pub use rstm_traits as traits;
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
    pub use rstm_traits::prelude::*;

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
