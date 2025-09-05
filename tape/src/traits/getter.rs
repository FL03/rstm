/*
    appellation: fetch <module>
    authors: @FL03
*/

/// The [`Get`] trait defines a basic interface for retrieving elements from a collection by
/// index or key.
pub trait Get<K> {
    type Output;

    fn get(&self, index: K) -> Option<&Self::Output>;
}
/// The [`GetMut`] trait extends the [`Get`] trait to provide mutable access to elements in a
/// collection.
pub trait GetMut<K>: Get<K> {
    /// Fetch a mutable reference to the element at the given index.
    fn get_mut(&mut self, index: K) -> Option<&mut Self::Output>;
}

/*
 ************* Implementations *************
*/

impl<I, T> Get<I> for [T]
where
    I: core::slice::SliceIndex<[T]>,
    I::Output: Sized,
{
    type Output = I::Output;

    fn get(&self, index: I) -> Option<&Self::Output> {
        <[T]>::get(self, index)
    }
}

impl<I, T> GetMut<I> for [T]
where
    I: core::slice::SliceIndex<[T]>,
    I::Output: Sized,
{
    fn get_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        <[T]>::get_mut(self, index)
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{Get, GetMut};
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;
    use core::cmp::Ord;

    impl<I, T> Get<I> for Vec<T>
    where
        I: core::slice::SliceIndex<[T]>,
        I::Output: Sized,
    {
        type Output = I::Output;

        fn get(&self, index: I) -> Option<&Self::Output> {
            <[T]>::get(self, index)
        }
    }

    impl<I, T> GetMut<I> for Vec<T>
    where
        I: core::slice::SliceIndex<[T]>,
        I::Output: Sized,
    {
        fn get_mut(&mut self, index: I) -> Option<&mut Self::Output> {
            <[T]>::get_mut(self, index)
        }
    }

    impl<Q, K, V> Get<Q> for BTreeMap<K, V>
    where
        K: Ord + core::borrow::Borrow<Q>,
        Q: Ord,
    {
        type Output = V;

        fn get(&self, index: Q) -> Option<&Self::Output> {
            <BTreeMap<K, V>>::get(self, &index)
        }
    }

    impl<Q, K, V> GetMut<Q> for BTreeMap<K, V>
    where
        K: Ord + core::borrow::Borrow<Q>,
        Q: Ord,
    {
        fn get_mut(&mut self, index: Q) -> Option<&mut Self::Output> {
            <BTreeMap<K, V>>::get_mut(self, &index)
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::{Get, GetMut};
    use core::hash::Hash;
    use std::collections::HashMap;

    impl<Q, K, V> Get<Q> for HashMap<K, V>
    where
        K: Eq + Hash + core::borrow::Borrow<Q>,
        Q: Eq + Hash,
    {
        type Output = V;

        fn get(&self, index: Q) -> Option<&Self::Output> {
            <HashMap<K, V>>::get(self, &index)
        }
    }

    impl<Q, K, V> GetMut<Q> for HashMap<K, V>
    where
        K: Eq + Hash + core::borrow::Borrow<Q>,
        Q: Eq + Hash,
    {
        fn get_mut(&mut self, index: Q) -> Option<&mut Self::Output> {
            <HashMap<K, V>>::get_mut(self, &index)
        }
    }
}
