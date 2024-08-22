/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Direction, Error, Head, State, Tail};
use core::cell::Cell;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// In-line with the Turing machine model, the [`Tape`] is a one-dimensional surface evenly
/// divided into cells capable of storing symbols. The tape is infinite in both directions
/// allowing the head, or actor, to move without bounds, extending the tape as needed.
///
///
/// Here, the tape employs the use of a [Vec] to store symbols while leveraging a
/// [usize] to keep track of the current position of the tape head. Moreover, the tape
/// stores the number of steps or operations taken by the tape head in a [Cell<usize>].
/// This is done to quantify the impact of operations whose directions are defined to
/// be [Direction::Stay]. Moving left and right within a linear space speaks directly
/// to a translation or shift in space, however, staying in place does not result in
/// any movement, shift, or translation within space. That being said, staying still
/// is an operation that does result in some change in-time.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tape<S = char> {
    cursor: usize,
    store: Vec<S>,
    ticks: Cell<usize>,
}

impl<S> Tape<S> {
    pub fn new() -> Self {
        Tape {
            cursor: 0,
            store: Vec::<S>::new(),
            ticks: Cell::default(),
        }
    }
    /// Constructs a new tape from an iterator.
    pub fn from_iter(iter: impl IntoIterator<Item = S>) -> Self {
        Tape {
            cursor: 0,
            store: Vec::from_iter(iter),
            ticks: Cell::default(),
        }
    }
    /// Constructs a new, empty tape with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Tape {
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
    /// Returns an immutable iterator over the symbols stored on the tape.
    pub fn iter(&self) -> core::slice::Iter<S> {
        self.store.iter()
    }
    /// Returns a mutable iterator over the symbols stored on the tape.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<S> {
        self.store.iter_mut()
    }
    /// Returns the number of steps or operations taken by the tape head;
    /// this provides a measure of the impact of operations whose directions are defined to be
    /// [Stay](Direction::Stay).
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
    pub fn write(&mut self, symbol: S) {
        if self.cursor < self.store.len() {
            self.store[self.cursor] = symbol;
        } else {
            self.store.push(symbol);
        }
    }

    fn shift(&mut self, direction: Direction) -> usize {
        self.on_update();
        self.cursor = direction.apply(self.cursor) % self.store.len();
        self.position()
    }

    pub fn update<Q>(self, direction: Direction, state: State<Q>, symbol: S) -> (Self, Head<Q, S>)
    where
        S: Clone,
    {
        let head = Head::new(state, symbol.clone());
        let mut tape = self;
        tape.write(symbol);
        tape.shift(direction);
        (tape, head)
    }

    pub fn update_inplace(
        &mut self,
        direction: Direction,
        symbol: S,
    ) {
        self.write(symbol);
        self.shift(direction);
    }

    pub fn apply_inplace<Q>(&mut self, tail: Tail<Q, S>) -> State<Q> {
        let Tail {
            direction,
            state,
            symbol,
        } = tail;
        self.update_inplace(direction, symbol);
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

impl Tape {
    pub fn from_str(input: &str) -> Tape {
        Tape {
            cursor: 0,
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
                b if i == self.cursor => write!(f, "[{b}]")?,
                _ => write!(f, "{c}")?,
            }
        }
        Ok(())
    }
}

impl<S> core::iter::Extend<S> for Tape<S> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = S>,
    {
        self.store.extend(iter);
    }
}

impl<S> core::iter::FromIterator<S> for Tape<S> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = S>,
    {
        Tape::from_iter(iter)
    }
}

impl<S> core::iter::IntoIterator for Tape<S> {
    type Item = S;
    type IntoIter = std::vec::IntoIter<S>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}

impl<S, I> core::ops::Index<I> for Tape<S>
where
    I: core::slice::SliceIndex<[S]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.store[index]
    }
}

impl<S, I> core::ops::IndexMut<I> for Tape<S>
where
    I: core::slice::SliceIndex<[S]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.store[index]
    }
}
