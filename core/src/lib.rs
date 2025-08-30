/*
    Appellation: rstm-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm-core
//!
//! The `rstm-core` crate provides the core functionality for the `rstm` library.
//!
//! ## Features
//!
//! ### Components
//!
//! - [x] Rules
//! - [x] States
//!
//! ### Tapes
//!
//! - [x] [`StdTape`](mem::std_tape::StdTape)
//! - [x] [HashTape](mem::hash_tape::HashTape)

#![allow(
    clippy::module_inception,
    clippy::new_ret_no_self,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![crate_name = "rstm_core"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

// extern crate rstm_state as state;
pub use rstm_state as state;

#[doc(inline)]
pub use self::{
    actors::prelude::*,
    error::{Error, Result},
    ops::prelude::*,
    rules::{Head, InstructionSet, Rule, Tail},
    state::{RawState, State},
    traits::prelude::*,
    types::prelude::*,
};

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

pub mod actors;
pub mod error;
pub mod mem;
pub mod rules;

pub mod ops {
    //! this modules defines additional operations used throughout the crate
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod apply;
    pub mod increment;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::apply::*;
        #[doc(inline)]
        pub use super::increment::*;
    }
}

pub mod traits {
    /// this modules provides various traits used throughout the library
    pub use self::prelude::*;

    pub mod convert;
    pub mod symbols;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::symbols::*;
    }
}

pub mod types {
    //! the `types` module provides various types used throughout the library, including
    //! [`Direction`], [`Head`], and [`Tail`].
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod direction;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::direction::*;
    }
}

pub mod prelude {
    pub use rstm_state::prelude::*;

    #[doc(no_inline)]
    pub use crate::actors::prelude::*;
    #[doc(no_inline)]
    pub use crate::mem::prelude::*;
    #[doc(no_inline)]
    pub use crate::ops::prelude::*;
    #[doc(no_inline)]
    pub use crate::rules::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
}
