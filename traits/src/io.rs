/*
    Appellation: io <module>
    Created At: 2026.01.11:13:54:16
    Contrib: @FL03
*/
/// [`Read`] trait defines an interface for objects capable of reading data into a buffer.
pub trait Read<T> {
    type Error: core::error::Error;
    type Output;

    fn read(&mut self, rhs: &mut [T]) -> Result<Self::Output, Self::Error>;
}

/// The [`Write`] trait defines an interface for objects that can perform write operations.
pub trait Write<Rhs> {
    type Error: core::error::Error;
    type Output;

    fn write(&mut self, rhs: &mut [Rhs]) -> Result<Self::Output, Self::Error>;
}
