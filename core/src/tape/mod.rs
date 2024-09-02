/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Tape
//!
//! Idealized Turing machines consider a tape, or memory, that is infinite in both directions.
//! This tape is a one-dimensional array of symbols manipulated by the tape head according to
//! some set of pre-defined rules.
pub use self::tape::StdTape;

pub(crate) mod tape;

#[cfg(feature = "std")]
pub mod hash_tape;

pub(crate) mod prelude {
    pub use super::tape::StdTape;
}

use core::option::Option;

#[doc(hidden)]
pub trait Mem {
    type Key;
    type Value;

    fn clear(&mut self);

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    fn insert(&mut self, key: Self::Key, value: Self::Value);

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

#[doc(hidden)]
pub trait Tape<A = char> {
    type Index;
    type Elem;

    fn clear(&mut self);

    fn get(&self, index: &Self::Index) -> Option<&A>;

    fn get_mut(&mut self, index: &Self::Index) -> Option<&mut A>;

    fn insert(&mut self, index: Self::Index, symbol: A);

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(feature = "alloc")]
impl<V> Mem for Vec<V> {
    type Key = usize;
    type Value = V;

    fn clear(&mut self) {
        Vec::clear(self);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        match key {
            key if *key < self.len() => Some(&self[*key]),
            _ => None,
        }
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        match key {
            key if *key < self.len() => Some(&mut self[*key]),
            _ => None,
        }
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) {
        Vec::insert(self, key, value);
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}

#[cfg(feature = "std")]
impl<K, V> Mem for HashMap<K, V>
where
    K: Eq + std::hash::Hash,
    V: Eq + std::hash::Hash,
{
    type Key = K;
    type Value = V;

    fn clear(&mut self) {
        HashMap::clear(self);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        HashMap::get(self, key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        HashMap::get_mut(self, key)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) {
        HashMap::insert(self, key, value);
    }

    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }

    fn len(&self) -> usize {
        HashMap::len(self)
    }
}
