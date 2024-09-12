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

use crate::{Direction, Error, Head, Rule, State, Tail};

/// [Handle] describes the step-by-step execution of a program; the trait is generalized
/// with the introduction of a single generic parameter, `T`, capable of sufficiently
/// representing any possible object that may be passed to the [handle] method.
///
/// This notion is particularly useful as it allows us to define the process using an actor,
/// before generically implementing the [Engine] trait for the [Executor] struct. Doing so
/// allows for further abstraction by considering the
pub trait Handle<T> {
    type Output;

    fn handle(&mut self, args: T) -> Self::Output;
}

#[doc(hidden)]
pub trait Engine<Q, S>: Handle<(Direction, State<Q>, S)> {
    fn load<I>(&mut self, program: I)
    where
        I: IntoIterator<Item = Rule<Q, S>>;

    fn run(&mut self) -> Result<(), Error>;
}

impl<Q, S> Handle<(Direction, State<Q>, S)> for Actor<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: crate::Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(&mut self, (direction, state, symbol): (Direction, State<Q>, S)) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> Handle<(Direction, Head<Q, S>)> for Actor<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: crate::Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(
        &mut self,
        (direction, Head { state, symbol }): (Direction, Head<Q, S>),
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> Handle<Tail<Q, S>> for Actor<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: crate::Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(
        &mut self,
        Tail {
            direction,
            state,
            symbol,
        }: Tail<Q, S>,
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<D, Q, S> Handle<D> for Executor<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: crate::Symbolic,
    Actor<Q, S>: Handle<D>,
{
    type Output = <Actor<Q, S> as Handle<D>>::Output;

    fn handle(&mut self, args: D) -> Self::Output {
        self.actor.handle(args)
    }
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

    fn run(&mut self) -> Result<(), crate::Error> {
        Executor::run(self)
    }
}
