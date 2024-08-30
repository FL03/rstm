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
    rules::{Program, Rule},
    state::State,
    tape::StdTape,
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
pub mod rules;
#[doc(hidden)]
pub mod shift;
pub mod state;
pub mod tape;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use crate::actors::prelude::*;
    pub use crate::error::Error;
    pub use crate::rules::prelude::*;
    pub use crate::shift::prelude::*;
    pub use crate::state::prelude::*;
    pub use crate::tape::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
}
