/*
    appellation: aliases <module>
    authors: @FL03
*/
use super::{Head, Tail};

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(feature = "alloc")]
pub(crate) type Rules<Q, S> = alloc::vec::Vec<crate::rules::Rule<Q, S>>;

#[cfg(feature = "std")]
/// A type alias for a [`HashMap`] with keys of type [`Head<Q, S>`] and values of type
/// [`Tail<Q, S>`].
pub type HeadMap<Q = usize, S = usize> = HashMap<Head<Q, S>, Tail<Q, S>>;
