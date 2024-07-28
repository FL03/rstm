/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{instruction::*, program::*};

pub(crate) mod instruction;
pub(crate) mod program;

pub(crate) mod prelude {
    pub use super::instruction::Instruction;
    pub use super::program::Program;
}

use crate::{Direction, State};

pub trait Rule {
    type Elem;
    type State;
}

pub trait Transition<Q, S>: Rule<Elem = S, State = Q> {
    fn direction(&self) -> Direction;

    fn current_state(&self) -> State<Q>;

    fn next_state(&self) -> State<Q>;

    fn symbol(&self) -> Self::Elem;

    fn write_symbol(&self) -> S;
}

pub trait Read<Q, S> {
    type State;

    fn current_state(&self) -> State<&'_ Q>;

    fn symbol(&self) -> &S;
}

pub trait Write<Q, S> {
    fn direction(&self) -> Direction;

    fn next_state(&self) -> State<&'_ Q>;

    fn write_symbol(&self) -> &S;
}

/*
 ************* Implementations *************
*/
impl<Q, S> Read<Q, S> for (State<Q>, S) {
    type State = Q;

    fn current_state(&self) -> State<&'_ Q> {
        self.0.to_view()
    }

    fn symbol(&self) -> &S {
        &self.1
    }
}

impl<Q, S> Read<Q, S> for crate::Head<Q, S> {
    type State = Q;

    fn current_state(&self) -> State<&'_ Q> {
        self.state.to_view()
    }

    fn symbol(&self) -> &S {
        &self.symbol
    }
}

impl<Q, S> Write<Q, S> for (Direction, State<Q>, S) {
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
