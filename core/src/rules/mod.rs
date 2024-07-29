/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    instruction::*,
    parts::*,
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

    pub type IndexedHead<Q> = Head<Q, usize>;
}

pub(crate) mod prelude {
    pub use super::instruction::Instruction;
    pub use super::parts::{Head, Tail};
    pub use super::program::Program;
}

use crate::{Direction, State, Symbolic};

pub trait Rule {
    type Elem;
    type State;
}

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

pub trait Header<Q, S> {
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

impl<A, Q, S> Transition<Q, S> for A
where
    A: Header<Q, S> + Directive<Q, S>,
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
        self.write_symbol()
    }
}

impl<Q, S> Header<Q, S> for Instruction<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.head.state.to_ref()
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

impl<Q, S> Header<Q, S> for crate::Head<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.state().to_ref()
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

impl<Q, S> Header<Q, S> for (State<Q>, S) {
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

    fn write_symbol(&self) -> &S {
        &self.2
    }
}
