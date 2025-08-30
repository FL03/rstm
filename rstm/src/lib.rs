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

#[cfg(feature = "actors")]
#[doc(inline)]
pub use rstm_actors as actors;
#[doc(inline)]
pub use rstm_core::*;

pub mod prelude {
    #[cfg(all(feature = "alloc", feature = "macros"))]
    pub use crate::program;
    #[cfg(all(feature = "std", feature = "macros"))]
    pub use crate::rulemap;
    pub use crate::state;
    #[cfg(feature = "actors")]
    pub use rstm_actors::prelude::*;
    #[doc(no_inline)]
    pub use rstm_core::prelude::*;
}
