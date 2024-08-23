/*
    Appellation: rstm <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm
//!
//! `rstm` is a Rust library dedicated to the construction and execution of Turing Machines.
//! The crate is designed to be flexible and easy to use while preserving the abstract nature
//! of the models.

// #![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::models::Turm;
#[doc(inline)]
pub use rstm_core::*;

#[doc(hidden)]
pub mod cspace;
pub mod models;

pub mod prelude {
    pub use super::models::prelude::*;

    pub use rstm_core::prelude::*;
}
