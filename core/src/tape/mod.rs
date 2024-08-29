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
/// [RawTape] defines the basic interface used for tape-like structures; i.e., a contiguous,
/// sequential array of elements.
pub trait RawTape {
    type Elem;

    private!();

    fn as_slice(&self) -> &[Self::Elem];

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

#[doc(hidden)]
/// [Tape] is a
pub trait Tape: RawTape {
    type Idx;

    fn clear(&mut self);

    fn get(&self, idx: &Self::Idx) -> Option<&Self::Elem>;

    fn get_mut(&mut self, idx: &Self::Idx) -> Option<&mut Self::Elem>;

    fn insert(&mut self, idx: Self::Idx, elem: Self::Elem);
}

/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use std::collections::HashMap;

impl<T> RawTape for [T] {
    type Elem = T;

    seal!();

    fn as_slice(&self) -> &[Self::Elem] {
        &self
    }

    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }

    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T> RawTape for Vec<T> {
    type Elem = T;

    seal!();

    fn as_slice(&self) -> &[Self::Elem] {
        Vec::as_slice(self)
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}

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
        HashMap::get_mut(self, &key)
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
