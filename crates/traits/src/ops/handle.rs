/*
    Appellation: handle <module>
    Created At: 2025.08.30:00:16:00
    Contrib: @FL03
*/

#[deprecated(since = "0.1.4", note = "Use the `TryExecute` trait instead")]
/// The [`Handle`] trait
pub trait Handle<T> {
    type Output;

    fn handle(&mut self, args: T) -> Self::Output;
}
