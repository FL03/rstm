/*
    Appellation: fsm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Head;

pub trait Automata {
    type Rule;
    type State;
    type Symbol;

    fn current_state(&self) -> Self::State;
}

pub enum Step<T> {
    Left(T),
    Right(T),
    Stay(T),
}

pub trait Tape {
    type Symbol;

    fn read(&self) -> Self::Symbol;

    fn write(&mut self, symbol: Self::Symbol);

    fn step(&mut self, step: Step<usize>);
}

pub struct Context<Q, S> {
    pub head: Head<Q, usize>,
    pub tape: S,
}
pub struct Actor {
    pub(crate) state: usize,
    pub(crate) tape: Vec<char>,
}