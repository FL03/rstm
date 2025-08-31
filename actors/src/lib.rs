/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This modules implements an [Actor] struct, which is a Turing machine with a moving head
//! (TMH).
//!
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::self_named_constructors,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate rstm_core as rstm;

#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! {
    "Either the `alloc` or `std` feature must be enabled for this crate to compile."
}

#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
}


#[doc(inline)]
pub use self::{tmh::TMH, error::*, exec::Executor, traits::*};

#[cfg(feature = "alloc")]
pub(crate) mod tmh;
#[cfg(feature = "alloc")]
pub(crate) mod exec;

pub mod error;

pub mod traits {
    //! the traits supporting the actors within the framework
    #[doc(inline)]
    pub use self::prelude::*;

    mod actor;
    mod engine;
    mod handle;

    mod prelude {
        #[doc(inline)]
        pub use super::actor::*;
        #[doc(inline)]
        pub use super::engine::*;
        #[doc(inline)]
        pub use super::handle::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use crate::tmh::TMH;
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use crate::exec::Executor;
    #[doc(inline)]
    pub use crate::traits::*;
}
