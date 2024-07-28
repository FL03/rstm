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
    cursor: usize,
    store: Vec<S>,
    ticks: Cell<usize>,
}

impl<S> StdTape<S> {
    pub fn new() -> Self {
        StdTape {
            cursor: 0,
            store: Vec::<S>::new(),
            ticks: Cell::default(),
        }
    }
    /// Constructs a new tape from an iterator.
    pub fn from_iter(iter: impl IntoIterator<Item = S>) -> Self {
        StdTape {
            cursor: 0,
            store: Vec::from_iter(iter),
            ticks: Cell::default(),
        }
    }
    /// Constructs a new, empty tape with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        StdTape {
            cursor: 0,
            store: Vec::<S>::with_capacity(capacity),
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
    /// Clears the tape, removing all elements.
    pub fn clear(&mut self) {
        self.store.clear();
    }
    /// Returns the number of elements in the tape.
    pub fn len(&self) -> usize {
        self.store.len()
    }
    /// Checks if the tape is empty; returns `true` if the tape is empty,
    /// `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    
    pub fn iter(&self) -> core::slice::Iter<S> {
        self.store.iter()
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

    /// Returns the current position of the tape head;
    pub fn position(&self) -> usize {
        self.cursor
    }

    /// Returns an owned reference to the current symbol on the tape
    pub fn read(&self) -> Result<&S, Error> {
        self.get(self.cursor)
            .ok_or(Error::index_out_of_bounds(self.cursor, self.len()))
    }
    ///
    pub fn write(&mut self, direction: Direction, symbol: S) {
        self.write_symbol(symbol);
        self.shift(direction);
        self.on_update();
    }

    ///
    fn write_symbol(&mut self, symbol: S) {
        if self.cursor < self.store.len() {
            self.store[self.cursor] = symbol;
        } else {
            self.store.push(symbol);
        }
    }

    fn shift(&mut self, direction: Direction) -> usize {
        self.on_update();
        self.cursor = direction.apply(self.cursor);
        self.position()
    }

    pub fn update<Q>(self, direction: Direction, state: State<Q>, symbol: S) -> (Self, Head<Q, S>)
    where
        S: Clone,
    {
        let head = Head::new(state, symbol.clone());
        let mut tape = self;
        tape.write(direction, symbol);
        tape.shift(direction);
        (tape, head)
    }

    pub fn update_inplace<Q>(&mut self, tail: Tail<Q, S>) -> State<Q> {
        let Tail {
            direction,
            state,
            symbol,
        } = tail;

        self.write(direction, symbol);
        self.shift(direction);
        state
    }

    fn on_update(&self) {
        self.ticks.set(self.ticks.get() + 1);
    }

    /// Given an index, return a reference to the symbol at that index;
    /// panics if the index is out of bounds.
    pub fn get<I>(&self, idx: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[S]>,
    {
        self.store.get(idx)
    }
    /// Removes and returns the last element of the tape, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<S> {
        self.store.pop()
    }
    /// Appends the given element to the back of the collection.
    pub fn push(&mut self, symbol: S) {
        self.store.push(symbol);
    }
}

impl StdTape {
    pub fn from_str(input: &str) -> StdTape {
        StdTape {
            cursor: 0,
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
                b if i == self.cursor => write!(f, "[{b}]")?,
                _ => write!(f, "{c}")?,
            }
        }
        Ok(())
    }
}

impl<S> core::iter::Extend<S> for StdTape<S> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = S>,
    {
        self.store.extend(iter);
    }
}

impl<S> core::iter::FromIterator<S> for StdTape<S> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = S>,
    {
        StdTape::from_iter(iter)
    }
}

impl<S> core::iter::IntoIterator for StdTape<S> {
    type Item = S;
    type IntoIter = std::vec::IntoIter<S>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}

impl<S, I> core::ops::Index<I> for StdTape<S>
where
    I: core::slice::SliceIndex<[S]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.store[index]
    }
}

impl<S, I> core::ops::IndexMut<I> for StdTape<S>
where
    I: core::slice::SliceIndex<[S]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.store[index]
    }
}
