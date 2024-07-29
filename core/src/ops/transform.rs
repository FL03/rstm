/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [`Transform`] is describes a binary operation capable of applying some transformation.
/// More commonly, the typical "rustic" manner of which an object is transformed is through
/// the [`map`] method, which applies a function to a value and returns a new value.
pub trait Transform<T> {
    type Output;
    /// [`Transform::transform`] is a method that takes a reference to `self` and a value of type 
    /// `T` and returns a value of type [`Transform::Output`].
    fn transform(&self, delta: T) -> Self::Output;
}
