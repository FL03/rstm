/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [`Shift`] describes a generalized shift operation;
/// w.r.t. Turing machines, Moore (1990) describes a shift operation as a _movement_ of the
/// tape head
pub trait Shift<T> {
    type Output;

    fn shift(&self, tape: &T) -> Self::Output;
}
/// [`Transform`] is describes a binary operation capable of applying some transformation.
/// More commonly, the typical "rustic" manner of which an object is transformed is through
/// the [`map`] method, which applies a function to a value and returns a new value.
pub trait Transform<T> {
    type Output;
    /// [`Transform::transform`] is a method that takes a reference to `self` and a value of type
    /// `T` and returns a value of type [`Transform::Output`].
    fn transform(&self, delta: T) -> Self::Output;
}
