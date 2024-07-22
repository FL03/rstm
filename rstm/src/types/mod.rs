/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    direction::Direction,
    parts::{Head, Tail},
    scope::Scope,
    tape::Tape,
};

pub mod direction;
pub mod parts;
pub mod scope;
pub mod tape;

pub(crate) mod prelude {
    pub use super::direction::Direction;
    pub use super::parts::{Head, Tail};
    pub use super::scope::Scope;
    pub use super::tape::Tape;
}

#[allow(unused)]
pub(crate) type HeadMap<T> = std::collections::HashMap<Head<T>, Tail<T>>;
