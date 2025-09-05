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
    type Key;

    fn clear(&mut self);

    fn get(&self, index: &Self::Key) -> Option<&A>;

    fn get_mut(&mut self, index: &Self::Key) -> Option<&mut A>;

    fn insert(&mut self, index: Self::Key, symbol: A);

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

/*
 ************* Implementations *************
*/

impl<T> RawTape for &T
where
    T: RawTape,
{
    type Elem = T::Elem;

    seal! {}
}

impl<T> RawTape for &mut T
where
    T: RawTape,
{
    type Elem = T::Elem;

    seal! {}
}

impl<T> RawTape for [T] {
    type Elem = T;

    seal! {}
}

impl<T, const N: usize> RawTape for [T; N] {
    type Elem = T;

    seal! {}
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{RawTape, Tape};
    use alloc::collections::{BTreeMap, BTreeSet};
    use alloc::vec::Vec;

    impl<T> RawTape for BTreeSet<T> {
        type Elem = T;

        seal! {}
    }

    impl<K, V> RawTape for BTreeMap<K, V> {
        type Elem = V;

        seal! {}
    }

    impl<T> RawTape for Vec<T> {
        type Elem = T;

        seal! {}
    }

    impl<T> Tape<T> for Vec<T> {
        type Key = usize;

        fn clear(&mut self) {
            Vec::clear(self);
        }

        fn get(&self, key: &Self::Key) -> Option<&T> {
            match key {
                key if *key < self.len() => Some(&self[*key]),
                _ => None,
            }
        }

        fn get_mut(&mut self, key: &Self::Key) -> Option<&mut T> {
            match key {
                key if *key < self.len() => Some(&mut self[*key]),
                _ => None,
            }
        }

        fn insert(&mut self, key: Self::Key, value: T) {
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
    use std::collections::{HashMap, HashSet};

    impl<T> RawTape for HashSet<T> {
        type Elem = T;

        seal! {}
    }

    impl<K, V> RawTape for HashMap<K, V> {
        type Elem = V;

        seal! {}
    }

    impl<K, V> Tape<V> for HashMap<K, V>
    where
        K: Eq + Hash,
        V: Eq + Hash,
    {
        type Key = K;

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
