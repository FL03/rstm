/*
    Appellation: rstm <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm
//!
#![crate_name = "rstm"]
// #![cfg_attr(not(feature = "std"), no_std)]
// #[cfg(feature = "alloc")]
// extern crate alloc;

pub use rstm_core::*;

#[doc(inline)]
pub use self::turing::TM;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod rules;
}

pub mod turing;

pub mod prelude {
    pub use crate::turing::TM;
    pub use rstm_core::prelude::*;
}
