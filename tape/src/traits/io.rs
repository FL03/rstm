/*
    Appellation: io <module>
    Created At: 2025.09.05:17:41:15
    Contrib: @FL03
*/

/// The [`Read`] trait provides an interface for implementing read operations on a tape or
/// memory.
pub trait Read {
    type Output;
}
/// [`Write`] operations for manipulating a cell (or set of cells) along the tape.
pub trait Write {}
