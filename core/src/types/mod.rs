/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::direction::Direction;

pub mod direction;

pub(crate) mod prelude {
    pub use super::direction::Direction;

    #[allow(unused)]
    pub(crate) use super::Idx;
}

/// A type alias generally used to represent the position of a value within a collection.
pub(crate) type Idx = usize;
