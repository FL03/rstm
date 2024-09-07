/*
    Appellation: snapshot <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::mem::RawMemory;
use crate::State;

pub struct Snapshot<'a, Q, M>
where
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
{
    pub fn state(&self) -> State<&'a Q> {
        self.state
    }
}
