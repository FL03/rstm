/*
    Appellation: rstm-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm-core
//!

// #![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    actors::Actor, error::Error, ops::prelude::*, rules::prelude::*, state::State, tape::Tape,
    traits::prelude::*, types::prelude::*,
};

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;

#[doc(hidden)]
pub mod actors;
pub mod error;
pub mod ops;
pub mod rules;
pub mod state;
pub mod tape;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use crate::actors::prelude::*;
    pub use crate::error::Error;
    pub use crate::ops::prelude::*;
    pub use crate::rules::prelude::*;
    pub use crate::state::prelude::*;
    pub use crate::tape::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
}
