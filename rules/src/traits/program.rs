/*
    appellation: program <module>
    authors: @FL03
*/
use crate::Rule;
use rstm_core::{Direction, Head, Tail};
use rstm_state::{RawState, State};

/// The [`Scope`] trait is used to describe objects containing information or references to the
/// current state and symbol of a Turing machine.
pub trait Scope<Q, S> {
    fn current_state(&self) -> &State<Q>;

    fn current_symbol(&self) -> &S;
}

/// [`Directive`] is a trait describing the `tail` of a typical Turing machine;
pub trait Directive<Q, S> {
    fn direction(&self) -> Direction;

    fn next_state(&self) -> &State<Q>;

    fn next_symbol(&self) -> &S;
}

/*
 ************* Implementations *************
*/

impl<Q, S> Scope<Q, S> for (State<Q>, S) {
    fn current_state(&self) -> &State<Q> {
        &self.0
    }

    fn current_symbol(&self) -> &S {
        &self.1
    }
}

impl<Q, S> Scope<Q, S> for Head<Q, S>
where
    Q: RawState,
{
    fn current_state(&self) -> &State<Q> {
        self.state()
    }

    fn current_symbol(&self) -> &S {
        &self.symbol
    }
}

impl<Q, S> Scope<Q, S> for Rule<Q, S>
where
    Q: RawState,
{
    fn current_state(&self) -> &State<Q> {
        self.state()
    }

    fn current_symbol(&self) -> &S {
        self.symbol()
    }
}

impl<Q, S> Directive<Q, S> for (Direction, State<Q>, S)
where
    Q: RawState,
{
    fn direction(&self) -> Direction {
        self.0
    }

    fn next_state(&self) -> &State<Q> {
        &self.1
    }

    fn next_symbol(&self) -> &S {
        &self.2
    }
}

impl<Q, S> Directive<Q, S> for Tail<Q, S>
where
    Q: RawState,
{
    fn direction(&self) -> Direction {
        self.direction
    }

    fn next_state(&self) -> &State<Q> {
        self.state()
    }

    fn next_symbol(&self) -> &S {
        self.symbol()
    }
}

impl<Q, S> Directive<Q, S> for Rule<Q, S>
where
    Q: RawState,
{
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn next_state(&self) -> &State<Q> {
        self.tail().state()
    }

    fn next_symbol(&self) -> &S {
        self.write_symbol()
    }
}
