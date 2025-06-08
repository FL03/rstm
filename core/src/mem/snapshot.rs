/*
    Appellation: snapshot <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::mem::RawMemory;
use crate::{RawState, State};

pub struct Snapshot<'a, Q, M>
where
    Q: RawState,
    M: RawMemory,
{
    pub state: State<&'a Q>,
    pub symbol: *mut M::Elem,
    pub tape: M,
}

impl<'a, Q, A, M> Snapshot<'a, Q, M>
where
    A: 'a,
    M: RawMemory<Elem = A>,
    Q: RawState,
{
    pub const fn state(&self) -> State<&'a Q> {
        self.state
    }
}
