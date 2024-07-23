/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Direction, FsmError, State, Tail};
use core::cell::Cell;

use super::Head;

#[allow(unused)]
#[doc(hidden)]
pub struct Slider<Q> {
    scope: usize,
    state: *const State<Q>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tape<S = char> {
    pos: usize,
    store: Vec<S>,
    ticks: Cell<usize>,
}

impl<S> Tape<S> {
    pub fn new() -> Self {
        Tape {
            pos: 0,
            store: Vec::<S>::new(),
            ticks: Cell::default(),
        }
    }
    /// Returns an owned reference to the store as a [slice](core::slice)
    pub fn as_slice(&self) -> &[S] {
        &self.store
    }
    /// Returns a mutable reference to the store as a [slice](core::slice)
    pub fn as_mut_slice(&mut self) -> &mut [S] {
        &mut self.store
    }
    /// Returns the number of elements in the tape.
    pub fn len(&self) -> usize {
        self.store.len()
    }
    /// Given an index, return a reference to the symbol at that index;
    /// panics if the index is out of bounds.
    pub fn get<I>(&self, idx: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[S]>,
    {
        self.store.get(idx)
    }

    pub fn ticks(&self) -> usize {
        self.ticks.get()
    }

    pub fn to_string(&self) -> String
    where
        S: core::fmt::Display,
    {
        format!("step ({}): {}", self.ticks(), self)
    }
    /// Removes and returns the last element of the tape, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<S> {
        self.store.pop()
    }
    /// Returns the current position of the tape head;
    pub fn position(&self) -> usize {
        self.pos
    }
    /// Appends the given element to the back of the collection.
    pub fn push(&mut self, symbol: S) {
        self.store.push(symbol);
    }
    /// Returns an owned reference to the current symbol on the tape
    pub fn read(&self) -> Result<&S, FsmError> {
        self.get(self.pos)
            .ok_or(FsmError::index_out_of_bounds(self.pos))
    }
    ///
    pub fn write(&mut self, symbol: S) {
        if self.pos < self.store.len() {
            self.store[self.pos] = symbol;
        } else {
            self.store.push(symbol);
        }
    }

    pub fn write_iter(&mut self, iter: impl Iterator<Item = S>) {
        for (i, symbol) in iter.enumerate() {
            if i < self.store.len() {
                self.store[i] = symbol;
            } else {
                self.store.push(symbol);
            }
        }
    }

    pub fn shift(self, direction: Direction) -> Self {
        self.on_update();
        Self {
            pos: direction.apply(self.pos),
            store: self.store,
            ticks: self.ticks,
        }
    }

    pub fn shift_left(self) -> Self {
        self.shift(Direction::Left)
    }

    pub fn shift_right(self) -> Self {
        self.shift(Direction::Right)
    }

    pub fn step(&mut self, direction: Direction) {
        self.pos = direction.apply(self.pos);
        self.on_update();
    }

    pub fn step_left(&mut self) {
        self.step(Direction::Left);
    }

    pub fn step_right(&mut self) {
        self.step(Direction::Right);
    }

    pub fn update<Q>(self, tail: Tail<Q, S>) -> (Self, Head<Q, S>)
    where
        S: Clone,
    {
        let Tail {
            direction,
            state,
            symbol,
        } = tail;
        let mut tape = self;
        tape.write(symbol.clone());
        tape.step(direction);
        tape.on_update();
        (tape, Head::new(state, symbol))
    }

    pub fn update_inplace<Q>(&mut self, tail: Tail<Q, S>) -> State<Q> {
        let Tail {
            direction,
            state,
            symbol,
        } = tail;
        self.write(symbol);
        self.step(direction);
        self.on_update();
        state
    }

    fn on_update(&self) {
        self.ticks.set(self.ticks.get() + 1);
    }
}

impl Tape {
    pub fn from_str(input: &str) -> Tape {
        Tape {
            pos: 0,
            store: input.chars().collect(),
            ticks: Cell::default(),
        }
    }
}

impl<S> AsRef<[S]> for Tape<S> {
    fn as_ref(&self) -> &[S] {
        &self.store
    }
}

impl<S> AsMut<[S]> for Tape<S> {
    fn as_mut(&mut self) -> &mut [S] {
        &mut self.store
    }
}

impl<S> core::borrow::Borrow<[S]> for Tape<S> {
    fn borrow(&self) -> &[S] {
        &self.store
    }
}

impl<S> core::borrow::BorrowMut<[S]> for Tape<S> {
    fn borrow_mut(&mut self) -> &mut [S] {
        &mut self.store
    }
}

impl<S> core::ops::Deref for Tape<S> {
    type Target = [S];

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<S> core::ops::DerefMut for Tape<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl<S> core::fmt::Display for Tape<S>
where
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for (i, c) in self.store.iter().enumerate() {
            match c {
                b if i == self.pos => write!(f, "[{b}]")?,
                _ => write!(f, "{c}")?,
            }
        }
        Ok(())
    }
}
