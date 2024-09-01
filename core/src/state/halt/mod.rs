/*
    Appellation: halt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Halting State
//!
//! For all intents and purposes, the halt state is an imaginary state not actually considered
//! by the machine's state space.
//!
#[doc(inline)]
pub use self::{state::Halt, wrap::HaltState};

pub(crate) mod state;
pub(crate) mod wrap;

use super::RawState;

#[doc(hidden)]
pub trait Haltable<Q> {
    type State: RawState<Q = Q>;

    private!();

    fn is_halted(&self) -> bool;
}

#[doc(hidden)]
pub trait HaltableExt<Q>: Haltable<Q> {
    fn get(self) -> Option<Q>;

    fn get_mut(&mut self) -> Option<&mut Q>;

    fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(Q) -> U,
        Self: Sized,
    {
        self.get().map(|state| f(state))
    }
}

/*
 ************* Implementations *************
*/
use crate::state::State;

impl<Q> Haltable<Q> for Halt<Q> {
    type State = State<Q>;

    seal!();

    fn is_halted(&self) -> bool {
        true
    }
}

impl<Q> Haltable<Q> for State<Halt<Q>> {
    type State = State<Q>;

    seal!();

    fn is_halted(&self) -> bool {
        true
    }
}
impl<Q> Haltable<Q> for State<Option<Q>> {
    type State = State<Q>;

    seal!();

    fn is_halted(&self) -> bool {
        self.get_ref().is_none()
    }
}

impl<Q> Haltable<Q> for Option<State<Q>> {
    type State = State<Q>;

    seal!();

    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

impl<Q> HaltableExt<Q> for Halt<Q> {
    fn get(self) -> Option<Q> {
        Some(self.0)
    }

    fn get_mut(&mut self) -> Option<&mut Q> {
        Some(&mut self.0)
    }
}

impl<Q> HaltableExt<Q> for Option<State<Q>> {
    fn get(self) -> Option<Q> {
        self.map(|state| state.get())
    }

    fn get_mut(&mut self) -> Option<&mut Q> {
        self.as_mut().map(|state| state.get_mut())
    }
}

impl<Q> HaltableExt<Q> for State<Option<Q>> {
    fn get(self) -> Option<Q> {
        self.get()
    }

    fn get_mut(&mut self) -> Option<&mut Q> {
        self.get_mut().as_mut()
    }
}
