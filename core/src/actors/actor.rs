/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::rules::Directive;
use crate::{Head, State, Symbolic};

/// [Actor] aptly describe the `TMH` model
pub struct Actor<Q, S> {
    pub head: Head<Q, usize>,
    pub tape: Vec<S>,
}

impl<Q, S> Actor<Q, S> {
    pub fn new(State(state): State<Q>, tape: impl IntoIterator<Item = S>) -> Self {
        Self {
            head: Head::new(State(state), 0),
            tape: Vec::from_iter(tape),
        }
    }

    pub fn handle<D>(&mut self, rule: D)
    where
        D: Directive<Q, S>,
        S: Symbolic,
    {
        self.write(rule.value().clone());
        self.head.shift_inplace(rule.direction());
    }

    pub fn is_halted(&self) -> bool {
        self.head.symbol >= self.tape.len()
    }

    pub fn len(&self) -> usize {
        self.tape.len()
    }

    pub fn read(&self) -> &S {
        &self.tape[self.head.symbol % self.len()]
    }

    pub fn write(&mut self, symbol: S) {
        if self.head.symbol < self.tape.len() {
            self.tape[self.head.symbol] = symbol;
        } else {
            self.tape.push(symbol);
        }
    }
}
