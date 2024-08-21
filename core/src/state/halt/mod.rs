/*
    Appellation: halt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This module is responsible for defining the [Halt] state of a Turing machine.
//!
//!
#[doc(inline)]
pub use self::{state::Halt, wrap::Halting};

pub(crate) mod state;
pub(crate) mod wrap;

use crate::state::RawState;

#[doc(hidden)]
pub trait Haltable<Q = String> {
    const HALT: bool = true;

    type State: RawState<Ctx = Q>;

    private!();
}
