/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Actors
//!
//! An actor describes an abstract model of computation that may find the solution to any
//! computable sequence, or algorithm.
#[doc(inline)]
pub use self::{actor::Actor, exec::Executor};

pub(crate) mod actor;
pub(crate) mod exec;

pub(crate) mod prelude {
    pub use super::actor::Actor;
    pub use super::exec::Executor;
}

use crate::{Direction, Error, Head, Rule, State};

#[doc(hidden)]
pub trait GetRule<Q, S> {
    fn get(&self, state: State<&Q>, symbol: &S) -> Option<&Rule<Q, S>>;

    fn get_mut(&mut self, state: State<&Q>, symbol: &S) -> Option<&mut Rule<Q, S>>;
}

#[doc(hidden)]
pub trait Dynamical<Q, F> {
    type Output;

    fn step(&mut self, f: F) -> Self::Output;
}

#[doc(hidden)]
pub trait Engine<Q, S> {
    fn load<I>(&mut self, program: I)
    where
        I: IntoIterator<Item = Rule<Q, S>>;

    fn handle(&mut self, direction: Direction, state: State<Q>, symbol: S) -> Head<Q, usize>;

    fn run(&mut self) -> Result<(), Error>;
}

impl<Q, S> Engine<Q, S> for Executor<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: crate::Symbolic,
{
    fn load<I>(&mut self, program: I)
    where
        I: IntoIterator<Item = crate::Rule<Q, S>>,
    {
        self.program.rules.clear();
        self.program.extend(program);
    }

    fn handle(&mut self, direction: Direction, state: State<Q>, symbol: S) -> Head<Q, usize> {
        self.actor.step(direction, state, symbol)
    }

    fn run(&mut self) -> Result<(), crate::Error> {
        Executor::run(self)
    }
}
