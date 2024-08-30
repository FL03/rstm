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

#[allow(unused)]
#[doc(hidden)]
mod engine;

pub(crate) mod prelude {
    pub use super::actor::Actor;
    pub use super::exec::Executor;
}

pub trait Program<Q, S> {
    fn get(&self, state: crate::State<&Q>, symbol: &S) -> Option<&crate::Rule<Q, S>>;

    fn get_mut(&mut self, state: crate::State<&Q>, symbol: &S) -> Option<&mut crate::Rule<Q, S>>;
}

#[doc(hidden)]
pub trait Runtime<Q, S> {
    fn load<I>(&mut self, program: I)
    where
        I: IntoIterator<Item = crate::Rule<Q, S>>;

    fn run(&mut self) -> Result<(), crate::Error>;
}

impl<Q, S> Runtime<Q, S> for Executor<Q, S>
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

    fn run(&mut self) -> Result<(), crate::Error> {
        Executor::run(self)
    }
}
