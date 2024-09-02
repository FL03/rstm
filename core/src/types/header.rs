/*
    Appellation: header <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Direction, Head};
use crate::State;

pub struct Tail<Q, S> {
    pub direction: Direction,
    pub head: Head<Q, S>,
}

impl<Q, S> Tail<Q, S> {
    pub fn new(direction: Direction, state: State<Q>, symbol: S) -> Self {
        Self {
            direction,
            head: Head::new(state, symbol),
        }
    }

    pub fn as_head_ref(&self) -> Head<&Q, &S> {
        self.head.to_ref()
    }

    pub fn state(&self) -> State<&'_ Q> {
        self.head.state()
    }

    pub fn symbol(&self) -> &S {
        self.head.symbol()
    }

    pub fn to_ref(&self) -> Tail<&'_ Q, &'_ S> {
        Tail {
            direction: self.direction,
            head: self.head.to_ref(),
        }
    }

    pub fn to_mut(&mut self) -> Tail<&'_ mut Q, &'_ mut S> {
        Tail {
            direction: self.direction,
            head: self.head.to_mut(),
        }
    }
}
