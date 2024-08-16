/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{direction::Direction, head::Head, tail::Tail};

pub mod direction;
pub mod head;
pub mod tail;

pub(crate) mod prelude {
    pub use super::direction::Direction;
    pub use super::head::Head;
    pub use super::tail::Tail;

    #[allow(unused)]
    pub(crate) use super::Idx;
}

/// A type alias generally used to represent the position of a value within a collection.
pub(crate) type Idx = usize;
///
pub type IndexedHead<Q> = Head<Q, Idx>;
