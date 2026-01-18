/*
    Appellation: io <module>
    Created At: 2026.01.11:13:54:16
    Contrib: @FL03
*/
/// [`Read`] establishes an interface for objects capable of reading data from internal
/// sources.
pub trait Read<T> {
    type Error: core::error::Error;

    fn read(&self) -> Result<&T, Self::Error>;
}

/// [`ReadBuf`] trait defines an interface for objects capable of reading data into a buffer.
pub trait ReadBuf<T> {
    type Error: core::error::Error;

    fn read(&mut self, rhs: &mut [T]) -> Result<T, Self::Error>;
}

/// The [`Write`] trait defines an interface for objects that can perform write operations.
pub trait Write<Rhs> {
    type Error: core::error::Error;
    type Output;

    fn write(&mut self, rhs: &mut [Rhs]) -> Result<Self::Output, Self::Error>;
}
