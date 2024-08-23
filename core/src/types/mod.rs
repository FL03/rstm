/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{head::Head, tail::Tail};

pub(crate) mod head;
pub(crate) mod tail;

#[doc(hidden)]
pub mod cursor;

pub(crate) mod prelude {
    pub use super::head::Head;
    pub use super::tail::Tail;
    pub use super::IndexedHead;

    #[allow(unused)]
    pub(crate) use super::Idx;
}

/// A type alias generally used to represent the position of a value within a collection.
pub(crate) type Idx = usize;
/// A type alias for a head which store an index as its symbol
pub type IndexedHead<Q> = Head<Q, Idx>;
