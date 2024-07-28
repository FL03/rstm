/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Direction, Error, Head, State, Tail};
use core::cell::Cell;

/// [StdTape] is an implementation of the traditional tape described by Turing machines.
///
/// The tape, often thought of as the memory of the machine, is a one-dimensional array
/// of symbols in-which the tape head can read and write symbols. Furthermore, the tape
/// is infinite in both directions, meaning that the tape head can move left or right.
/// While this setup is largely hypothetical, it is a useful abstraction for understanding
/// the capabilities of Turing machines.
///
/// Here, the [StdTape] employs the use of a [Vec] to store symbols while leveraging a
/// [usize] to keep track of the current position of the tape head. Moreover, the tape
/// stores the number of steps or operations taken by the tape head in a [Cell<usize>].
/// This is done to quantify the impact of operations whose directions are defined to
/// be [Direction::Stay]. Moving left and right within a linear space speaks directly
/// to a translation or shift in space, however, staying in place does not result in
/// any movement, shift, or translation within space.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct StdTape<S = char> {
    pos: usize,
    store: Vec<S>,
    ticks: Cell<usize>,
}

impl<S> StdTape<S> {
    pub fn new() -> Self {
        StdTape {
            pos: 0,
            store: Vec::<S>::new(),
            ticks: Cell::default(),
        }
    }
    /// Returns a raw pointer to the store.
    pub fn as_ptr(&self) -> *const S {
        self.store.as_ptr()
    }
    /// Returns a mutable raw pointer to the store.
    pub fn as_mut_ptr(&mut self) -> *mut S {
        self.store.as_mut_ptr()
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
    pub fn read(&self) -> Result<&S, Error> {
        self.get(self.pos)
            .ok_or(Error::index_out_of_bounds(self.pos, self.len()))
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

impl StdTape {
    pub fn from_str(input: &str) -> StdTape {
        StdTape {
            pos: 0,
            store: input.chars().collect(),
            ticks: Cell::default(),
        }
    }
}

impl<S> AsRef<[S]> for StdTape<S> {
    fn as_ref(&self) -> &[S] {
        &self.store
    }
}

impl<S> AsMut<[S]> for StdTape<S> {
    fn as_mut(&mut self) -> &mut [S] {
        &mut self.store
    }
}

impl<S> core::borrow::Borrow<[S]> for StdTape<S> {
    fn borrow(&self) -> &[S] {
        &self.store
    }
}

impl<S> core::borrow::BorrowMut<[S]> for StdTape<S> {
    fn borrow_mut(&mut self) -> &mut [S] {
        &mut self.store
    }
}

impl<S> core::ops::Deref for StdTape<S> {
    type Target = [S];

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<S> core::ops::DerefMut for StdTape<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl<S> core::fmt::Display for StdTape<S>
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
