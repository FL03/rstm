/*
    Appellation: rstm <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm
//!
//! `rstm` is a Rust library dedicated to the construction and execution of Turing Machines.
//! The crate is designed to be flexible and easy to use while preserving the abstract nature
//! of the models.
//!
//! Actors are one of the primary focuses of the library, designed to essentially mimic the
//! behavior of a smart-contract in reverse. Actors provide an _actionable_ or computable
//! surface for workloads to be executed on.
//!
#![crate_name = "rstm"]
// #![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

pub use rstm_core::*;

#[doc(inline)]
pub use self::turing::TM;

#[macro_use]
pub(crate) mod macros {}

pub mod turing;

pub mod prelude {
    pub use crate::turing::TM;
    pub use rstm_core::prelude::*;
}
