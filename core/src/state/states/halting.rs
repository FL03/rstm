/*
    Appellation: halting <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;

#[doc(hidden)]
pub trait Halter {
    const HALT: bool = true;

    private!();
}
#[doc(hidden)]
pub enum Halting<T> {
    Continue(T),
    Halt(T),
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

impl<T> Halter for State<Halt<T>> {
    seal!();
}
