/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod registry;
pub mod scope;
pub mod store;

pub(crate) mod prelude {

    pub(crate) use super::Idx;
}

/// A type alias generally used to represent the position of a value within a collection.
pub(crate) type Idx = usize;
