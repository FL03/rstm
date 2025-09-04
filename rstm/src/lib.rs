//! # rstm
//!
//! `rstm` is a Rust library dedicated to the construction and execution of Turing Machines.
//! The crate is designed to be flexible and easy to use while preserving the abstract nature
//! of the models.
#![allow(
    clippy::module_inception,
    clippy::new_ret_no_self,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![crate_name = "rstm"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "macros")]
#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod program;
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
#[cfg(feature = "tape")]
#[doc(inline)]
pub use rstm_tape as tape;

pub mod prelude {
    #[cfg(feature = "actors")]
    pub use rstm_actors::prelude::*;
    #[doc(no_inline)]
    pub use rstm_core::prelude::*;
    #[cfg(feature = "rules")]
    pub use rstm_rules::prelude::*;
    #[cfg(feature = "tape")]
    pub use rstm_tape::prelude::*;
}
