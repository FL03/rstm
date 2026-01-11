/*
    Appellation: rstm-state <library>
    Created At: 2025.08.29:22:43:22
    Contrib: @FL03
*/
//! The [`state`](self) module provides abstractions and implementations for managing state
//! within the `rstm` framework.
//!
#![crate_type = "lib"]
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
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
    #[macro_use]
    pub(crate) mod state;
}
// modules
pub mod error;
// modules (private)
mod state;

mod impls {
    mod impl_halt;
    mod impl_state;
    mod impl_state_ops;
    mod impl_state_repr;
}

mod traits {
    #[doc(inline)]
    pub use self::{convert::*, halting::*, raw_state::*};

    mod convert;
    mod halting;
    mod raw_state;
}

mod types {
    #[doc(inline)]
    pub use self::aliases::*;

    mod aliases;
}
// re-exports
#[doc(inline)]
pub use self::{error::*, state::*, traits::*, types::*};
// prelude
#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "macros")]
    pub use crate::state;
    pub use crate::state::*;
    pub use crate::traits::*;
    pub use crate::types::*;
}
