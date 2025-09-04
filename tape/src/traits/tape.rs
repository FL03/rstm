/*
    Appellation: tape <module>
    Created At: 2025.08.30:17:47:24
    Contrib: @FL03
*/
/// The [`RawTape`] trait establishes a common interface for all tape-like objects.
pub trait RawTape {
    type Elem;

    private!();
}
/// The [`Tape`] trait extends the [`RawTape`] trait to provide additional functionality
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
mod impl_alloc {
    use super::{RawTape, Tape};
    use alloc::vec::Vec;

    impl<A> RawTape for Vec<A> {
        type Elem = A;

        seal!();
    }
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
}
#[cfg(feature = "std")]
mod impl_std {
    use super::{RawTape, Tape};

    use core::hash::Hash;
    use std::collections::HashMap;

    impl<K, A> RawTape for HashMap<K, A> {
        type Elem = A;

        seal!();
    }

    impl<K, V> Tape<V> for HashMap<K, V>
    where
        K: Eq + Hash,
        V: Eq + Hash,
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
}
