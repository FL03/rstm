/*
    Appellation: raw_tape <module>
    Created At: 2026.01.16:11:51:52
    Contrib: @FL03
*/
use super::RawCell;

/// The [`RawTape`] trait establishes a higher-kinded interface for tape-like structures
/// composed of cells.
pub trait RawTape<T>
where
    Self::Cell<T>: RawCell<Elem = T>,
{
    type Cell<V>;

    /// returns a view of the tape as a slice of cells
    fn as_slice(&self) -> &[Self::Cell<T>];
    /// Returns the length of the tape.
    fn len(&self) -> usize;
    /// Returns true if the tape is empty.
    fn is_empty(&self) -> bool;
    /// Gets a reference to a cell at the specified position.
    fn get(&self, idx: usize) -> Option<&Self::Cell<T>>;
}

pub trait RawTapeMut<T>: RawTape<T>
where
    Self::Cell<T>: RawCell<Elem = T>,
{
    /// Gets a mutable reference to a cell at the specified position.
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Cell<T>>;
}

/*
 ************* Implementations *************
*/

impl<T, C> RawTape<T> for [C]
where
    C: RawCell<Elem = T>,
{
    type Cell<V> = C;

    fn as_slice(&self) -> &[Self::Cell<T>] {
        self
    }

    fn len(&self) -> usize {
        <[C]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[C]>::is_empty(self)
    }

    fn get(&self, idx: usize) -> Option<&Self::Cell<T>> {
        <[C]>::get(self, idx)
    }
}

impl<T, C> RawTape<T> for &[C]
where
    C: RawCell<Elem = T>,
{
    type Cell<V> = C;

    fn as_slice(&self) -> &[Self::Cell<T>] {
        self
    }

    fn len(&self) -> usize {
        <[C]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[C]>::is_empty(self)
    }

    fn get(&self, idx: usize) -> Option<&Self::Cell<T>> {
        <[C]>::get(self, idx)
    }
}

impl<T, C> RawTape<T> for &mut [C]
where
    C: RawCell<Elem = T>,
{
    type Cell<V> = C;

    fn as_slice(&self) -> &[Self::Cell<T>] {
        self
    }

    fn len(&self) -> usize {
        <[C]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[C]>::is_empty(self)
    }

    fn get(&self, idx: usize) -> Option<&Self::Cell<T>> {
        <[C]>::get(self, idx)
    }
}

impl<const N: usize, T, C> RawTape<T> for [C; N]
where
    C: RawCell<Elem = T>,
{
    type Cell<V> = C;

    fn as_slice(&self) -> &[Self::Cell<T>] {
        self
    }

    fn len(&self) -> usize {
        <[C]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[C]>::is_empty(self)
    }

    fn get(&self, idx: usize) -> Option<&Self::Cell<T>> {
        <[C]>::get(self, idx)
    }
}

#[cfg(feature = "alloc")]
impl<T, C> RawTape<T> for alloc::vec::Vec<C>
where
    C: RawCell<Elem = T>,
{
    type Cell<V> = C;

    fn as_slice(&self) -> &[Self::Cell<T>] {
        self
    }

    fn len(&self) -> usize {
        <[C]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[C]>::is_empty(self)
    }

    fn get(&self, idx: usize) -> Option<&Self::Cell<T>> {
        <[C]>::get(self, idx)
    }
}
