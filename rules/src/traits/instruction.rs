/*
    appellation: transition <module>
    authors: @FL03
*/
use crate::rule::Rule;
use rstm_core::{Direction, Head, Symbol, Tail};
use rstm_state::{RawState, State};

/// The [`Scope`] trait is used to describe objects containing information or references to the
/// current state and symbol of a Turing machine.
pub trait Scope<Q, S>
where
    Q: RawState,
{
    fn current_state(&self) -> &State<Q>;

    fn current_symbol(&self) -> &S;
}

/// [`Directive`] is a trait describing the `tail` of a typical Turing machine;
pub trait Directive<Q, S>
where
    Q: RawState,
{
    fn direction(&self) -> Direction;

    fn next_state(&self) -> &State<Q>;

    fn next_symbol(&self) -> &S;
}

/// The [`Instruction`] trait defines the expected behaviors of a particular rule within a
/// Turing machine program.
pub trait Instruction<Q, S>
where
    Q: RawState,
{
    /// returns a copy of the direction of the head
    fn direction(&self) -> Direction;
    /// returns a reference to the current state of the Turing machine
    fn current_state(&self) -> &State<Q>;
    /// returns a reference to the next state of the Turing machine
    fn next_state(&self) -> &State<Q>;
    /// returns a reference to the current symbol under the head
    fn symbol(&self) -> &S;
    /// returns a reference to the symbol to be written by the head
    fn write_symbol(&self) -> &S;
    /// returns an instance of [`Head`] containing references to the current state and symbol
    fn head(&self) -> Head<&Q, &S> {
        Head {
            state: self.current_state().view(),
            symbol: self.symbol(),
        }
    }
    /// returns an instance of [`Tail`] containing references to the next state and symbol
    fn tail(&self) -> Tail<&Q, &S> {
        Tail {
            direction: self.direction(),
            next_state: self.next_state().view(),
            write_symbol: self.write_symbol(),
        }
    }
    /// returns an instance of [`Rule`] containing references to the states and symbols within
    fn as_rule(&self) -> Rule<&Q, &S> {
        Rule {
            head: self.head(),
            tail: self.tail(),
        }
    }
}

/*
 ************* Implementations *************
*/
impl<A, Q, S> Instruction<Q, S> for A
where
    A: Scope<Q, S> + Directive<Q, S>,
    Q: RawState,
    S: Symbol,
{
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn current_state(&self) -> &State<Q> {
        self.current_state()
    }

    fn next_state(&self) -> &State<Q> {
        self.next_state()
    }

    fn symbol(&self) -> &S {
        self.current_symbol()
    }

    fn write_symbol(&self) -> &S {
        self.next_symbol()
    }
}

impl<Q, S> Scope<Q, S> for (State<Q>, S)
where
    Q: RawState,
{
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
