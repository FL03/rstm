/*
    Appellation: fsm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Direction, Head, State, Symbolic, Tail};

pub trait Automata {
    type Rule;
    type State;
    type Symbol;

    fn current_state(&self) -> Self::State;
}

pub trait Store {
    type Elem;

    fn get(&self, index: usize) -> &Self::Elem;

    fn set(&mut self, index: usize, symbol: Self::Elem);
}

pub trait Memory {
    type Elem;

    fn read(&self) -> &Self::Elem;

    fn write(&mut self, symbol: Self::Elem);
}

pub trait Directive<Q, S> {
    fn direction(&self) -> crate::Direction;

    fn state(&self) -> State<&'_ Q>;

    fn symbol(&self) -> &S;
}

pub struct Actor<Q, S> {
    pub head: Head<Q, usize>,
    pub tape: Vec<S>,
}

impl<Q, S> Actor<Q, S> where S: Symbolic {
    pub fn new(State(state): State<Q>, tape: impl IntoIterator<Item = S>) -> Self {
        Self {
            head: Head::new(State(state), 0),
            tape: Vec::from_iter(tape),
        }
    }

    pub fn handle<D>(&mut self, rule: D) where D: Directive<Q, S> {
        self.write(rule.symbol().clone());
        self.head.shift_inplace(rule.direction());
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