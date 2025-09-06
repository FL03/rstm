/*
    Appellation: io <module>
    Created At: 2025.09.05:17:41:15
    Contrib: @FL03
*/

/// The [`Read`] trait provides an interface for implementing read operations on a tape or
/// memory.
pub trait Read<T> {
    type Output;

    fn read(&mut self, buf: &mut [T]) -> Self::Output;
}

/// [`Write`] operations for manipulating a cell (or set of cells) along the tape.
pub trait Write<T> {
    type Output;

    fn write(&mut self, buf: &[T]) -> Self::Output;
}
