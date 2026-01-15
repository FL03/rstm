/*
    Appellation: io <module>
    Created At: 2026.01.11:13:54:16
    Contrib: @FL03
*/
/// [`ReadInto`] trait defines an interface for objects capable of reading data into a buffer.
pub trait ReadInto<T> {
    type Buf<_T>: ?Sized;
    type Output;

    fn read(&mut self, rhs: &mut Self::Buf<T>) -> Self::Output;
}
/// The [`WriteInto`] trait defines an interface for objects that can perform write operations.
pub trait WriteInto<Rhs> {
    type Buf<'a, _T>: ?Sized;
    type Output;

    fn write(&mut self, rhs: &mut Self::Buf<'_, Rhs>) -> Self::Output;
}
