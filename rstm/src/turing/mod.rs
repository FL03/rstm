/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Turing Machine ([TM])
//!
//! ### Overview
//!
//! A Turing machine is a mathematical model describing the generalization of computation using
//! a set of symbols and a [tape](Tape). Assuming an infinite tape, the machine can read, write, and move linearly
//! across the tape. The machine uses a set of pre-defined rules to determine the next state and symbol.
//!
#[doc(inline)]
pub use self::{context::Context, model::TM};

pub(crate) mod context;
pub(crate) mod model;

pub(crate) mod prelude {
    pub use super::model::TM;
}

#[doc(hidden)]
pub trait Turing {}
