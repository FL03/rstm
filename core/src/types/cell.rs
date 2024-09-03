/*
    Appellation: cell <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Head, State};
use crate::tape::RawTape;

pub struct Cell<Q, A> {
    pub index: isize,
    pub state: State<Q>,
    pub symbol: A,
}

pub struct Snapshot<'a, Q, M>
where
    M: RawTape,
{
    pub index: isize,
    pub head: Head<&'a Q, *const M::Elem>,
    pub tape: M,
}

impl<Q, A> Cell<Q, A> {
    pub fn new(index: isize, state: State<Q>, symbol: A) -> Self {
        Self { index, state, symbol }
    }
}
