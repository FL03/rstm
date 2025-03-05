/*
    Appellation: increment <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Decrement] is a trait that provides a common interface for decrementing values; i.e.,
/// subtracting one from a value.
pub trait Decrement {
    fn decrement(self) -> Self;
}

/// [DecrementAssign] is a trait that provides a common interface for decrementing values in
/// place.
pub trait DecrementAssign {
    fn decrement_assign(&mut self);
}

/// [Increment] is a trait that provides a common interface for incrementing values; i.e.,
/// adding one to a value.
pub trait Increment {
    fn increment(self) -> Self;
}

/// [IncrementAssign] is a trait that provides a common interface for incrementing values in
/// place.
pub trait IncrementAssign {
    fn increment_assign(&mut self);
}

/// [Incremental] is a trait that provides a common interface for incrementing and decrementing values.
pub trait Incremental: Decrement + Increment + DecrementAssign + IncrementAssign {}

/*
 ************* Implementations *************
 */

impl<T> Decrement for T
where
    T: num::One + core::ops::Sub<Output = T>,
{
    fn decrement(self) -> Self {
        self - T::one()
    }
}

impl<T> DecrementAssign for T
where
    T: num::One + core::ops::SubAssign,
{
    fn decrement_assign(&mut self) {
        *self -= T::one();
    }
}

impl<T> Increment for T
where
    T: num::One + core::ops::Add<Output = T>,
{
    fn increment(self) -> Self {
        self + T::one()
    }
}

impl<T> IncrementAssign for T
where
    T: num::One + core::ops::AddAssign,
{
    fn increment_assign(&mut self) {
        *self += T::one();
    }
}

impl<T> Incremental for T where T: Decrement + Increment + DecrementAssign + IncrementAssign {}
