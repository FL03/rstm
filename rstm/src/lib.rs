/*
    Appellation: rstm <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm
//!
//! `rstm` is a Rust library dedicated to the construction and execution of Turing Machines.
//! The crate is designed to be flexible and easy to use while preserving the abstract nature
//! of the models.

#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rstm"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "macros")]
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod rules;
}

#[doc(inline)]
pub use rstm_core::*;

pub mod prelude {
    #[cfg(all(feature = "std", feature = "macros"))]
    pub use crate::rulemap;
    #[cfg(all(feature = "alloc", feature = "macros"))]
    pub use crate::ruleset;
    pub use crate::state;
    #[doc(no_inline)]
    pub use rstm_core::prelude::*;
}
