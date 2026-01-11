/*
    Appellation: io <module>
    Created At: 2026.01.11:13:54:16
    Contrib: @FL03
*/
/// [`ReadBuf`] trait defines an interface for objects that can perform read operations.
pub trait ReadBuf<T> {
    type Buf<'a, _T>: ?Sized;
    type Output;

    fn read(&mut self, rhs: &mut Self::Buf<'_, T>) -> Self::Output;
}
/// The [`WriteBuf`] trait defines an interface for objects that can perform write operations.
pub trait WriteBuf<Rhs> {
    type Buf<'a, _T>: ?Sized;
    type Output;

    fn write(&mut self, rhs: &Self::Buf<'_, Rhs>) -> Self::Output;
}
