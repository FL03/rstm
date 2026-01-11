/*
    Appellation: increment <module>
    Contrib: @FL03
*/
/// An unary operator defining an incremental operation
pub trait Increment {
    fn increment(self) -> Self;
}
/// A mutable unary operator defining a self-incrementing operation
pub trait IncrementMut {
    fn increment_mut(&mut self);
}
/// An unary operator defining a decremental operation
pub trait Decrement {
    fn decrement(self) -> Self;
}
/// A mutable unary operator defining a self-decrementing operation
pub trait DecrementMut {
    fn decrement_mut(&mut self);
}

/*
 ************* Implementations *************
*/
use num_traits::One;

impl<T> Decrement for T
where
    T: One + core::ops::Sub<T, Output = T>,
{
    fn decrement(self) -> Self {
        self - T::one()
    }
}

impl<T> DecrementMut for T
where
    T: One + core::ops::SubAssign<T>,
{
    fn decrement_mut(&mut self) {
        *self -= T::one()
    }
}

impl<T> Increment for T
where
    T: One + core::ops::Add<T, Output = T>,
{
    fn increment(self) -> Self {
        self + T::one()
    }
}

impl<T> IncrementMut for T
where
    T: One + core::ops::AddAssign<T>,
{
    fn increment_mut(&mut self) {
        *self += T::one()
    }
}
