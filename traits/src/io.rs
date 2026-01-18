/*
    Appellation: io <module>
    Created At: 2026.01.11:13:54:16
    Contrib: @FL03
*/
/// [`Reader`] establishes an interface for objects capable of reading data from internal
/// sources.
pub trait Reader<T> {
    type Error: core::error::Error;

    fn read(&self) -> Result<&T, Self::Error>;
}

/// [`Read`] trait defines an interface for objects capable of reading data into a buffer.
pub trait Read<Rhs> {
    type Output;
    type Error: core::error::Error;

    fn read(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The [`Write`] trait defines an interface for objects that can perform write operations.
pub trait Write<Rhs> {
    type Error: core::error::Error;
    type Output;

    fn write(&mut self, rhs: &mut [Rhs]) -> Result<Self::Output, Self::Error>;
}
