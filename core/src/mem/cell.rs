/*
    Appellation: cell <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::State;

/// [Cell] is a struct
pub struct Cell<Q, A> {
    pub index: usize,
    pub state: State<Q>,
    pub symbol: A,
}

impl<Q, A> Cell<Q, A> {
    pub fn state(&self) -> State<&Q> {
        self.state.view()
    }

    pub fn symbol(&self) -> &A {
        &self.symbol
    }
}
