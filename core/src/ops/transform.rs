/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// The [`Transform`] trait generically describes objects capable of applying some transformation.
/// From a "rustic" point of view, the trait simply provides an additional reference point for
/// the `map` method, which is a method that applies a function to a value and returns a new value.
///  
/// [`Transform::transform`] is a method that takes a reference to `self` and a value of type `T`
/// and returns a value of type [`Transform::Output`].
pub trait Transform<T> {
    type Output;

    fn transform(&self, delta: T) -> Self::Output;
}
