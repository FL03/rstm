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

pub mod hash_tape;

pub(crate) mod prelude {
    pub use super::tape::StdTape;
}

use core::option::Option;

#[doc(hidden)]
pub trait RawIndex {
    private!();
}

pub trait Index: RawIndex {
    fn increment(self) -> Self;

    fn decrement(self) -> Self;
}

pub trait HashIndex: Index + core::cmp::Eq + core::hash::Hash {}

#[doc(hidden)]
pub trait RawTape {
    type Elem;

    private!();
}

#[doc(hidden)]
pub trait Tape<A = char>: RawTape<Elem = A> {
    type Index;

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
impl<A> RawTape for Vec<A> {
    type Elem = A;

    seal!();
}

#[cfg(feature = "std")]
impl<K, A> RawTape for HashMap<K, A> {
    type Elem = A;

    seal!();
}

#[cfg(feature = "alloc")]
impl<V> Tape<V> for Vec<V> {
    type Index = usize;

    fn clear(&mut self) {
        Vec::clear(self);
    }

    fn get(&self, key: &Self::Index) -> Option<&V> {
        match key {
            key if *key < self.len() => Some(&self[*key]),
            _ => None,
        }
    }

    fn get_mut(&mut self, key: &Self::Index) -> Option<&mut V> {
        match key {
            key if *key < self.len() => Some(&mut self[*key]),
            _ => None,
        }
    }

    fn insert(&mut self, key: Self::Index, value: V) {
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
impl<K, V> Tape<V> for HashMap<K, V>
where
    K: Eq + std::hash::Hash,
    V: Eq + std::hash::Hash,
{
    type Index = K;

    fn clear(&mut self) {
        HashMap::clear(self);
    }

    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        HashMap::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) {
        HashMap::insert(self, key, value);
    }

    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }

    fn len(&self) -> usize {
        HashMap::len(self)
    }
}

macro_rules! impl_index {
    (@impl $T:ty) => {
        impl RawIndex for $T {
            seal!();
        }
    };
    ($($T:ty),* $(,)?) => {
        $(
            impl_index!(@impl $T);
        )*
    };
}

impl_index!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);

impl<T> Index for T
where
    T: RawIndex + core::ops::Add<Output = T> + core::ops::Sub<Output = T> + num::One,
{
    fn increment(self) -> Self {
        self + T::one()
    }

    fn decrement(self) -> Self {
        self - T::one()
    }
}

impl<T> HashIndex for T where T: Index + core::cmp::Eq + core::hash::Hash {}
