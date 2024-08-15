/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{instruction::*, parts::*, program::*};

pub(crate) mod instruction;
pub(crate) mod program;

#[doc(hidden)]
pub mod entry;

pub mod parts {
    pub use self::{head::*, tail::*};

    pub(crate) mod head;
    pub(crate) mod tail;

    pub type IndexedHead<Q> = Head<Q, usize>;
}

pub(crate) mod prelude {
    pub use super::instruction::Instruction;
    pub use super::parts::{Head, Tail};
    pub use super::program::Program;
}

use crate::{Direction, State, Symbolic};

pub trait Transition<Q, S>
where
    S: Symbolic,
{
    fn direction(&self) -> Direction;

    fn current_state(&self) -> State<&'_ Q>;

    fn next_state(&self) -> State<&'_ Q>;

    fn symbol(&self) -> &S;

    fn write_symbol(&self) -> &S;
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

impl<Q, S> Scope<Q, S> for Instruction<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.head.state.view()
    }

    fn symbol(&self) -> &S {
        &self.symbol()
    }
}

impl<Q, S> Directive<Q, S> for Instruction<Q, S> {
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.tail().next_state()
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
        self.next_state()
    }

    fn value(&self) -> &S {
        &self.write_symbol()
    }
}

impl<Q, S> Scope<Q, S> for (State<Q>, S) {
    fn current_state(&self) -> State<&'_ Q> {
        self.0.view()
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
        self.1.view()
    }

    fn value(&self) -> &S {
        &self.2
    }
}
