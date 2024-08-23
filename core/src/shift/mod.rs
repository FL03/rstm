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

/// [Directional] 
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

/// [`Shift`] describes a generalized shift operation;
/// w.r.t. Turing machines, Moore (1990) describes a shift operation as a _movement_ of the
/// tape head
pub trait Shift<T> {
    type Output;

    fn shift(&self, step: T) -> Self::Output;
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
