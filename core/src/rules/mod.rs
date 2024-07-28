/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    instruction::*,
    parts::{Head, Tail},
    program::*,
};

pub(crate) mod instruction;
pub(crate) mod program;

#[doc(hidden)]
pub mod entry;

pub mod parts {
    pub use self::{head::*, tail::*};

    pub(crate) mod head;
    pub(crate) mod tail;
}

pub(crate) mod prelude {
    pub use super::instruction::Instruction;
    pub use super::parts::{Head, Tail};
    pub use super::program::Program;
}

use crate::{Direction, State};

pub trait RuleT {
    type Elem;
    type State;
}

pub trait Transition<Q, S>: RuleT<Elem = S, State = Q> {
    fn direction(&self) -> Direction;

    fn current_state(&self) -> State<Q>;

    fn next_state(&self) -> State<Q>;

    fn symbol(&self) -> Self::Elem;

    fn write_symbol(&self) -> S;
}

pub trait Scope<Q, S> {
    fn current_state(&self) -> State<&'_ Q>;

    fn symbol(&self) -> &S;
}

pub trait Directive<Q, S> {
    fn direction(&self) -> Direction;

    fn next_state(&self) -> State<&'_ Q>;

    fn write_symbol(&self) -> &S;
}

/*
 ************* Implementations *************
*/

impl<Q, S> Scope<Q, S> for Instruction<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.head.state.to_view()
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

    fn write_symbol(&self) -> &S {
        &self.write_symbol()
    }
}

impl<Q, S> Scope<Q, S> for crate::Head<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.state.to_view()
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

    fn write_symbol(&self) -> &S {
        &self.write_symbol()
    }
}

impl<Q, S> Scope<Q, S> for (State<Q>, S) {
    fn current_state(&self) -> State<&'_ Q> {
        self.0.to_view()
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
        self.1.to_view()
    }

    fn write_symbol(&self) -> &S {
        &self.2
    }
}
