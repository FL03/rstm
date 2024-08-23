/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    builders::{ProgramBuilder, RuleBuilder},
    program::Program,
    rule::Rule,
};

pub(crate) mod program;
pub(crate) mod rule;

pub mod workload;

#[doc(hidden)]
pub(crate) mod builders {
    pub use self::{program::ProgramBuilder, rule::RuleBuilder};

    mod program;
    mod rule;
}

pub(crate) mod prelude {
    pub use super::program::Program;
    pub use super::rule::Rule;
    pub use super::{Directive, Scope, Transition};
}

use crate::{Direction, Head, State, Symbolic, Tail};

pub trait Transition<Q, S> {
    fn direction(&self) -> Direction;

    fn current_state(&self) -> State<&'_ Q>;

    fn next_state(&self) -> State<&'_ Q>;

    fn symbol(&self) -> &S;

    fn write_symbol(&self) -> &S;

    fn head(&self) -> Head<&Q, &S> {
        Head {
            state: self.current_state(),
            symbol: self.symbol(),
        }
    }

    fn tail(&self) -> Tail<&Q, &S> {
        Tail {
            direction: self.direction(),
            state: self.next_state(),
            symbol: self.write_symbol(),
        }
    }

    fn as_rule(&self) -> Rule<&Q, &S> {
        Rule {
            head: self.head(),
            tail: self.tail(),
        }
    }
}

/// The [`Scope`] trait is used to describe objects containing information or references to the
/// current state and symbol of a Turing machine.
pub trait Scope<Q, S> {
    fn current_state(&self) -> State<&'_ Q>;

    fn symbol(&self) -> &S;
}

/// [`Directive`] is a trait describing the `tail` of a typical Turing machine;
pub trait Directive<Q, S> {
    fn direction(&self) -> Direction;

    fn next_state(&self) -> State<&'_ Q>;

    fn value(&self) -> &S;
}

/*
 ************* Implementations *************
*/

impl<A, Q, S> Transition<Q, S> for A
where
    A: Scope<Q, S> + Directive<Q, S>,
    S: Symbolic,
{
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn current_state(&self) -> State<&'_ Q> {
        self.current_state()
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.next_state()
    }

    fn symbol(&self) -> &S {
        self.symbol()
    }

    fn write_symbol(&self) -> &S {
        self.value()
    }
}

impl<Q, S> Scope<Q, S> for Rule<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.head.state.to_ref()
    }

    fn symbol(&self) -> &S {
        &self.symbol()
    }
}

impl<Q, S> Directive<Q, S> for Rule<Q, S> {
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.tail().state()
    }

    fn value(&self) -> &S {
        &self.write_symbol()
    }
}

impl<Q, S> Scope<Q, S> for crate::Head<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.state()
    }

    fn symbol(&self) -> &S {
        &self.symbol
    }
}

impl<Q, S> Directive<Q, S> for crate::Tail<Q, S> {
    fn direction(&self) -> Direction {
        self.direction
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.state()
    }

    fn value(&self) -> &S {
        &self.symbol()
    }
}

impl<Q, S> Scope<Q, S> for (State<Q>, S) {
    fn current_state(&self) -> State<&'_ Q> {
        self.0.to_ref()
    }

    fn symbol(&self) -> &S {
        &self.1
    }
}

impl<Q, S> Directive<Q, S> for (Direction, State<Q>, S) {
    fn direction(&self) -> Direction {
        self.0
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.1.to_ref()
    }

    fn value(&self) -> &S {
        &self.2
    }
}
