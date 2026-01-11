/*
    Appellation: io <module>
    Created At: 2026.01.11:13:54:16
    Contrib: @FL03
*/
/// [`Read`] trait defines an interface for objects that can perform read operations.
pub trait Read<Rhs> {
    type Output;

    fn read(&self, rhs: Rhs) -> Self::Output;
}
/// The [`Write`] trait defines an interface for objects that can perform write operations.
pub trait Write<Rhs> {
    type Output;

    fn write(&mut self, rhs: Rhs) -> Self::Output;
}