/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{direction::Direction, head::Head, tail::Tail, tape::Tape};

pub mod direction;
pub mod head;
#[doc(hidden)]
pub mod scope;
#[doc(hidden)]
pub mod symbol;
pub mod tail;
pub mod tape;

pub(crate) mod prelude {
    pub use super::direction::Direction;
    pub use super::head::Head;
    pub use super::tail::Tail;
    pub use super::tape::Tape;
    pub use super::Registry;
}

/// A registry of [head](Head) and [tail](Tail) pairs
pub type Registry<Q = String, S = char> = std::collections::HashMap<Head<Q, S>, Tail<Q, S>>;
