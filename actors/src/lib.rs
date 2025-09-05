/*
    Appellation: rstm-actors <library>
    Created At: 2025.09.03:18:46:32
    Contrib: @FL03
*/
//! The [`actors`](self) module establishes a framework for defining and managing Turing
//! machines
//!
//! ## Overview
//!
//! Here, we define the concept of an _actor_ as a fundamental computational entity capable of
//! executing actions based on a set of rules or stimuli. This abstraction allows us to define
//! a distinct separation between the _actor_ and the _rulespace_, where the actor operates
//! independently of the specific logic that governs its behavior. By isolating the actor from
//! the rulespace, we can create a more modular and flexible system that can adapt to different
//! sets of rules without altering the core functionality of the actor itself.
//!
//! That being said, the design of an actor
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
pub use self::{engine::prelude::*, error::*, tmh::TMH, traits::*};

pub mod engine;
#[cfg(feature = "alloc")]
pub(crate) mod tmh;

pub mod error;

pub mod traits {
    //! the traits supporting the actors within the framework
    #[doc(inline)]
    pub use self::prelude::*;

    mod actor;
    mod handle;

    mod prelude {
        #[doc(inline)]
        pub use super::actor::*;
        #[doc(inline)]
        pub use super::handle::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[doc(inline)]
    pub use crate::engine::prelude::*;
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use crate::tmh::TMH;
    #[doc(inline)]
    pub use crate::traits::*;
}
