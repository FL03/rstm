/*
    Appellation: shift <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::direction::Direction;

pub(crate) mod direction;

pub(crate) mod prelude {
    pub use super::direction::Direction;
}

pub trait Directional {
    fn direction(&self) -> Direction;
}
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
    T: Clone + Into<Direction>,
{
    fn as_direction(&self) -> Direction {
        self.clone().into()
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
