/*
    Appellation: point <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::RawPoint;

/// A point in a space.
///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BasePoint<S>
where
    S: RawPoint,
{
    pub(crate) data: S,
}

impl<S, A> BasePoint<S>
where
    S: RawPoint<Elem = A>,
{
    pub fn new(data: S) -> Self {
        Self { data }
    }

    
}
