/*
    Appellation: rstm-tape <library>
    Created At: 2025.08.30:16:50:58
    Contrib: @FL03
*/
//! # rstm-core
//!
//! The `rstm-core` crate provides the core functionality for the `rstm` library.
//!
//! ## Features
#![allow(
    clippy::module_inception,
    clippy::new_ret_no_self,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate rstm_core as rstm;

#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
}

#[doc(inline)]
pub use self::{
    error::{Error, Result},
    tape::StdTape,
    traits::prelude::*,
};

pub mod cell;
pub mod error;
pub mod snapshot;
pub mod store;
pub mod tape;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod fetch;
    pub mod memory;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::fetch::*;
        #[doc(inline)]
        pub use super::memory::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::snapshot::*;
    pub use crate::store::*;
    pub use crate::tape::prelude::*;
    pub use crate::traits::prelude::*;
}
