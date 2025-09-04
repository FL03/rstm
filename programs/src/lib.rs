/*
    Appellation: rstm-programs <library>
    Created At: 2025.08.30:18:45:29
    Contrib: @FL03
*/
//! The [`programs`](self) module provides various structures and utilities for defining and 
//! managing Turing machine programs, including rules, rule spaces, and program execution.
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
    error::*,
    traits::*,
    types::*,
};

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::program::*;
#[cfg(feature = "std")]
#[doc(inline)]
pub use self::rule_map::RuleMap;

#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! {
    "Either the `alloc` or `std` feature must be enabled to use this crate."
}

pub mod error;

#[cfg(feature = "alloc")]
pub mod program;
#[cfg(feature = "std")]
pub mod rule_map;

pub mod traits {
    //! the traits defining compatible rules within the framework
    #[doc(inline)]
    pub use self::prelude::*;

    mod rulespace;

    mod prelude {
        #[doc(inline)]
        pub use super::rulespace::*;
    }
}

mod types {
    //! types essential to the construction of rules, programs, and other related objects
    #[doc(inline)]
    pub use self::prelude::*;

    mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }

    mod aliases {
        #[cfg(feature = "std")]
        use rstm_core::{Head, Tail};

        #[cfg(feature = "alloc")]
        pub(crate) type RuleVec<Q, S> = alloc::vec::Vec<rstm_core::Rule<Q, S>>;

        /// A type alias for a [`HashMap`](std::collections::HashMap) with keys of type [`Head<Q, S>`] and values of type
        /// [`Tail<Q, S>`].
        #[cfg(feature = "std")]
        pub type HeadMap<Q = usize, S = usize> = std::collections::HashMap<Head<Q, S>, Tail<Q, S>>;
    }
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::traits::*;
    pub use crate::types::*;

    #[cfg(feature = "alloc")]
    pub use crate::program::Program;
    #[cfg(feature = "std")]
    pub use crate::rule_map::RuleMap;
}
