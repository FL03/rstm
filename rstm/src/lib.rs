/*
    Appellation: rstm <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{error::FsmError, state::State, traits::prelude::*, turing::TM, types::prelude::*};

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;

pub mod error;
pub mod rules;
pub mod state;
pub mod traits;
pub mod turing;
pub mod types;

pub mod prelude {
    pub use crate::error::FsmError;
    pub use crate::rules::prelude::*;
    pub use crate::state::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::turing::prelude::*;
    pub use crate::types::prelude::*;
}
