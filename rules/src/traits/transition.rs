/*
    appellation: transition <module>
    authors: @FL03
*/
use super::{Directive, Scope};
use crate::{Head, Rule, Tail};
use rstm_core::{Direction, Symbolic};
use rstm_state::{RawState, State};

/// The [`Transition`] trait defines the expected behaviors of a particular rule within a
/// Turing machine program.
pub trait Transition<Q, S>
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

impl<A, Q, S> Transition<Q, S> for A
where
    A: Scope<Q, S> + Directive<Q, S>,
    Q: RawState,
    S: Symbolic,
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
