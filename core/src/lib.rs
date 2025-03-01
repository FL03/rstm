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
#![allow(clippy::new_ret_no_self)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    actors::{Actor, Executor, Handle},
    error::{Error, Result},
    ops::prelude::*,
    rules::{Rule, RuleSet},
    state::{Halt, State},
    traits::prelude::*,
    types::prelude::*,
};

#[allow(unused_macros)]
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod get;
    #[macro_use]
    pub mod rules;
    #[macro_use]
    pub mod wrap;
}
#[doc(hidden)]
#[macro_use]
pub(crate) mod seal;

pub mod actors;
pub mod error;
pub mod mem;
pub mod rules;
pub mod state;

pub mod ops {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod apply;
    pub mod increment;

    pub(crate) mod prelude {
        pub use super::apply::*;
        pub use super::increment::*;
    }
}

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod convert;
    pub mod symbols;

    pub(crate) mod prelude {
        pub use super::convert::*;
        pub use super::symbols::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod direction;
    pub mod head;
    pub mod tail;

    #[doc(hidden)]
    pub mod transition;

    pub(crate) mod prelude {
        pub use super::direction::*;
        pub use super::head::*;
        pub use super::tail::*;
    }
}

pub mod prelude {
    pub use super::actors::prelude::*;
    pub use super::error::Error;
    pub use super::mem::prelude::*;
    pub use super::ops::prelude::*;
    pub use super::rules::prelude::*;
    pub use super::state::prelude::*;
    pub use super::traits::prelude::*;
    pub use super::types::prelude::*;
}
