/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This modules implements an [Actor] struct, which is a Turing machine with a moving head
//! (TMH).
//!
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

#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! {
    "Either the `alloc` or `std` feature must be enabled for this crate to compile."
}

#[doc(inline)]
pub use self::{actor::Actor, error::*, exec::Executor, traits::*};

#[cfg(feature = "alloc")]
pub(crate) mod actor;
#[cfg(feature = "alloc")]
pub(crate) mod exec;

pub mod error;

pub mod traits {
    //! the traits supporting the actors within the framework
    #[doc(inline)]
    pub use self::prelude::*;

    mod engine;
    mod handle;

    mod prelude {
        #[doc(hidden)]
        pub use super::engine::Engine;
        #[doc(inline)]
        pub use super::handle::Handle;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use crate::actor::Actor;
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use crate::exec::Executor;
    #[doc(inline)]
    pub use crate::traits::*;
}
