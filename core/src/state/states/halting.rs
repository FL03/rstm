/*
    Appellation: halting <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{RawState, State, Stateful};

#[doc(hidden)]
pub trait Halting {
    const HALT: bool = true;

    private!();
    
}


pub struct Halt<T>(pub T);

impl<T> Halt<T> {
    pub fn new(halt: T) -> Self {
        Self(halt)
    }
    pub fn into_inner(self) -> T {
        self.0
    }
    pub fn as_ref(&self) -> &T {
        &self.0
    }
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<Q> RawState for Halt<Q> {
    type Ctx = Q;
}

impl<T> Stateful<T> for Halt<T> {
    type State = Halt<T>;
}

impl<T> Halting for Halt<T> {
    seal!();
}

impl<T> Halting for State<Halt<T>> {
    seal!();
}
