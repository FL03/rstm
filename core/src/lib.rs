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
//! - [x] [StdTape]
//! - [x] [HashTape](tape::hash_tape::HashTape)

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::module_inception, clippy::new_ret_no_self)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    actors::{Actor, Executor, Handle},
    error::{Error, Result},
    ops::prelude::*,
    rules::{Rule, RuleSet},
    state::{Halt, RawState, State, Stated},
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
pub mod state;

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
    pub mod head;
    pub mod tail;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::direction::*;
        #[doc(inline)]
        pub use super::head::*;
        #[doc(inline)]
        pub use super::tail::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use super::error::*;

    #[doc(no_inline)]
    pub use super::actors::prelude::*;
    #[doc(no_inline)]
    pub use super::mem::prelude::*;
    #[doc(no_inline)]
    pub use super::ops::prelude::*;
    #[doc(no_inline)]
    pub use super::rules::prelude::*;
    #[doc(no_inline)]
    pub use super::state::prelude::*;
    #[doc(no_inline)]
    pub use super::traits::prelude::*;
    #[doc(no_inline)]
    pub use super::types::prelude::*;
}
