/*
    Appellation: convert <module>
    Created At: 2025.12.15:21:29:39
    Contrib: @FL03
*/
use crate::{Direction, Head, Tail};

/// Converts to a [`Head`] by reference.
pub trait AsHead<Q, A> {
    fn as_head(&self) -> Head<Q, A>;
}
/// Consumes the caller to convert it into a [`Head`].
pub trait IntoHead<Q, A> {
    fn into_head(self) -> Head<Q, A>;
}

/// Converts a type into a [`Tail`] by reference.
pub trait AsTail<Q, A> {
    fn as_tail(&self) -> Tail<Q, A>;
}
/// A consuming trait for converting a type into a [`Tail`].
pub trait IntoTail<Q, A> {
    fn into_tail(self) -> Tail<Q, A>;
}
/// [`IntoDirection`] is a simple conversion trait for consuming types to turn into a [`Direction`].
pub trait IntoDirection {
    fn into_direction(self) -> Direction;
}
/*
 ************* Implementations *************
*/
impl<T> IntoDirection for T
where
    T: Into<Direction>,
{
    fn into_direction(self) -> Direction {
        self.into()
    }
}

impl<Q, A, T> IntoHead<Q, A> for T
where
    T: Into<Head<Q, A>>,
{
    fn into_head(self) -> Head<Q, A> {
        self.into()
    }
}

impl<Q, A, T> IntoTail<Q, A> for T
where
    T: Into<Tail<Q, A>>,
{
    fn into_tail(self) -> Tail<Q, A> {
        self.into()
    }
}
