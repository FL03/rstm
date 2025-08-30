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
mod macros {
    #[cfg(feature = "rules")]
    #[macro_use]
    pub(crate) mod rules;
}

#[cfg(feature = "actors")]
#[doc(inline)]
pub use rstm_actors as actors;
#[doc(inline)]
pub use rstm_core::*;
#[cfg(feature = "rules")]
#[doc(inline)]
pub use rstm_rules as rules;

pub mod prelude {
    #[cfg(all(feature = "macros", feature = "std"))]
    pub use crate::{program, rulemap};
    #[cfg(feature = "macros")]
    pub use crate::{rule, rules};
    #[cfg(feature = "actors")]
    pub use rstm_actors::prelude::*;
    #[doc(no_inline)]
    pub use rstm_core::prelude::*;
    #[cfg(feature = "rules")]
    pub use rstm_rules::prelude::*;
}
