/*
    Appellation: shift <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [`Shift`] describes a generalized shift operation;
/// w.r.t. Turing machines, Moore (1990) describes a shift operation as a _movement_ of the
/// tape head
pub trait Shift<T> {
    type Output;

    fn shift(&self, tape: &T) -> Self::Output;
}
