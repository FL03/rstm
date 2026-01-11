/*
    Appellation: handle <module>
    Created At: 2025.08.30:00:16:00
    Contrib: @FL03
*/

/// The [`Handle`] trait
pub trait Handle<T> {
    type Output;

    fn handle(&mut self, args: T) -> Self::Output;
}
