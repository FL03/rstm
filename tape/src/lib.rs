//! This crate focuses on providing various tape-related implementations and abstractions for
//! generalizing cell-based memory systems in Rust.
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
}
// modules
pub mod cell;
pub mod error;
// modules (private)
mod tape_base;

mod impls {
    mod impl_tape_base;
}

mod traits {
    #[doc(inline)]
    pub use self::{cellular::*, raw_data::*};

    mod cellular;
    mod raw_data;
}

mod types {}
// re-exports
#[doc(inline)]
pub use self::{cell::Cell, error::*, tape_base::*, traits::*};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::cell::*;
    pub use crate::tape_base::*;
    pub use crate::traits::*;
}
