/*
    Appellation: slider <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code)]
use crate::State;

pub struct Slider<Q, S> {
    state: State<*const Q>,
    symbol: *const S,
}

impl<Q, S> Slider<Q, S> {
    pub fn new(State(state): State<*const Q>, symbol: *const S) -> Self {
        Self {
            state: State(state),
            symbol,
        }
    }

    pub fn state(&self) -> State<*const Q> {
        self.state
    }

    pub fn symbol(&self) -> *const S {
        self.symbol
    }
}
