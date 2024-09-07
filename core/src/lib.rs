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
#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    actors::Actor,
    error::Error,
    rules::{Rule, Ruleset},
    state::State,
    traits::prelude::*,
    types::prelude::*,
};

#[allow(unused_macros)]
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod fmt;
    #[macro_use]
    pub mod rules;
    #[macro_use]
    pub mod states;
}
#[macro_use]
pub(crate) mod seal;

pub mod actors;
pub mod error;
pub mod mem;
pub mod rules;
pub mod state;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use super::actors::prelude::*;
    pub use super::error::Error;
    pub use super::mem::prelude::*;
    pub use super::rules::prelude::*;
    pub use super::state::prelude::*;
    pub use super::traits::prelude::*;
    pub use super::types::prelude::*;
}
