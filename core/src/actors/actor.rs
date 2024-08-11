/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Executor;
use crate::rules::{Directive, Program};
use crate::{Head, State, Symbolic};

/// [Actor] aptly describe the `TMH` model
pub struct Actor<Q, S> {
    /// the input alphabet
    pub alpha: Vec<S>,
    pub ptr: Head<Q, usize>,
}

impl<Q, S> Actor<Q, S> {
    pub fn new(State(state): State<Q>, tape: impl IntoIterator<Item = S>) -> Self {
        Self {
            alpha: Vec::from_iter(tape),
            ptr: Head::new(State(state), 0),
        }
    }

    pub fn from_state(State(state): State<Q>) -> Self {
        Self::new(State(state), Vec::new())
    }

    pub fn from_tape(tape: impl IntoIterator<Item = S>) -> Self
    where
        Q: Default,
    {
        Self::new(State::default(), tape)
    }

    pub const fn head(&self) -> &Head<Q, usize> {
        &self.ptr
    }

    pub fn state(&self) -> State<&Q> {
        self.ptr.state()
    }

    pub fn handle<D>(&mut self, rule: &D) -> Head<&Q, &S>
    where
        D: Directive<Q, S>,
        S: Symbolic,
    {
        self.write(*rule.value());
        self.ptr.shift_inplace(rule.direction());
        Head {
            state: self.ptr.state.to_ref(),
            symbol: self.read(),
        }
    }

    pub fn run(self, program: Program<Q, S>) -> Executor<Q, S>
    where
        Q: Clone + Default + PartialEq + 'static,
        S: Symbolic,
    {
        Executor::from_actor(self).with_program(program)
    }

    pub fn is_empty(&self) -> bool {
        self.alpha.is_empty()
    }

    pub fn is_halted(&self) -> bool
    where
        Q: 'static,
    {
        self.ptr.state.is_halt()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.alpha.len()
    }

    pub fn read(&self) -> &S {
        &self.alpha[self.ptr.symbol % self.len()]
    }

    pub fn write(&mut self, symbol: S) {
        let head = self.head();
        if head.symbol < self.len() {
            self.alpha[self.ptr.symbol] = symbol;
        } else {
            self.alpha.push(symbol);
        }
    }

    fn get(&mut self, index: Head<Q, usize>) -> &S where S: Clone + Default {
        if index.symbol >= self.len() {
            let diff = index.symbol - self.len();
            let ext = vec![S::default(); diff + 1];
            self.alpha.extend(ext);
        }
        &self.alpha[index.symbol % self.len()]
    }
}

