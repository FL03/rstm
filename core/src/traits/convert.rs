/*
    Appellation: convert <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Direction;

/// The [AsDirection] trait provides a convience method for converting a type into a [Direction].
pub trait AsDirection {
    fn as_direction(&self) -> Direction;
}
/// The [IntoDirection] trait provides a convience method for converting a type into a [Direction].
pub trait IntoDirection {
    fn into_direction(self) -> Direction;
}

/*
 ************* Implementations *************
*/
impl<T> AsDirection for T
where
    T: Clone + IntoDirection,
{
    fn as_direction(&self) -> Direction {
        self.clone().into_direction()
    }
}

impl<T> IntoDirection for T
where
    T: Into<Direction>,
{
    fn into_direction(self) -> Direction {
        self.into()
    }
}
