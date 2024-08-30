/*
    Appellation: halt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This module is responsible for defining the [Halt] state of a Turing machine.
//!
//!
#[doc(inline)]
pub use self::{state::Halt, wrap::HaltState};

pub(crate) mod state;
pub(crate) mod wrap;

use super::RawState;

#[doc(hidden)]
pub trait Haltable<Q> {
    const HALT: bool = true;

    type State: RawState<Inner = Q>;

    private!();
}

/*
 ************* Implementations *************
*/
use crate::state::State;

impl<Q> Haltable<Q> for Halt<Q> {
    type State = State<Q>;

    seal!();
}

impl<Q> Haltable<Q> for State<Halt<Q>> {
    type State = State<Q>;

    seal!();
}
