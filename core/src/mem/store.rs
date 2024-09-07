/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [RawMemory] is a trait that provides a common interface for memory storage.
pub trait RawMemory {
    type Elem;

    private!();

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;

    fn to_vec(&self) -> Vec<Self::Elem>
    where
        Self::Elem: Clone;
}

/// [Sequential] extends the base trait [RawMemory] to provide sequential access to memory.
pub trait Sequential: RawMemory {
    fn as_ptr(&self) -> *const Self::Elem;

    fn as_mut_ptr(&mut self) -> *mut Self::Elem;

    fn as_slice(&self) -> &[Self::Elem];

    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

pub trait Memory: RawMemory {
    type Key;

    fn get(&self, index: &Self::Key) -> Option<&Self::Elem>;

    fn get_mut(&mut self, index: &Self::Key) -> Option<&mut Self::Elem>;
}

pub trait MemoryMut: Memory {
    fn clear(&mut self);

    fn insert(&mut self, index: usize, elem: Self::Elem);

    fn remove(&mut self, index: usize) -> Option<Self::Elem>;
}

/*
 ************* Implementations *************
*/
impl<T> RawMemory for [T] {
    type Elem = T;

    seal!();

    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }

    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        <[T]>::to_vec(self)
    }
}

impl<T> Memory for [T] {
    type Key = usize;

    fn get(&self, index: &usize) -> Option<&T> {
        <[T]>::get(self, *index)
    }

    fn get_mut(&mut self, index: &usize) -> Option<&mut T> {
        <[T]>::get_mut(self, *index)
    }
}

impl<T> Sequential for [T] {
    fn as_ptr(&self) -> *const T {
        <[T]>::as_ptr(self)
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        <[T]>::as_mut_ptr(self)
    }

    fn as_slice(&self) -> &[T] {
        &self
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{Memory, RawMemory, Sequential};
    use alloc::vec::Vec;

    impl<T> RawMemory for Vec<T> {
        type Elem = T;

        seal!();

        fn is_empty(&self) -> bool {
            Vec::is_empty(self)
        }

        fn len(&self) -> usize {
            Vec::len(self)
        }

        fn to_vec(&self) -> Vec<T>
        where
            T: Clone,
        {
            self.clone()
        }
    }

    impl<T> Memory for Vec<T> {
        type Key = usize;

        fn get(&self, index: &usize) -> Option<&T> {
            if *index < self.len() {
                Some(&self[*index])
            } else {
                None
            }
        }

        fn get_mut(&mut self, index: &usize) -> Option<&mut T> {
            if *index < self.len() {
                Some(&mut self[*index])
            } else {
                None
            }
        }
    }

    impl<T> Sequential for Vec<T> {
        fn as_ptr(&self) -> *const T {
            Vec::as_ptr(self)
        }

        fn as_mut_ptr(&mut self) -> *mut T {
            Vec::as_mut_ptr(self)
        }

        fn as_slice(&self) -> &[T] {
            Vec::as_slice(self)
        }

        fn as_mut_slice(&mut self) -> &mut [T] {
            Vec::as_mut_slice(self)
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::{Memory, RawMemory};
    use std::collections::HashMap;

    impl<K, V> RawMemory for HashMap<K, V> {
        type Elem = V;

        seal!();

        fn is_empty(&self) -> bool {
            HashMap::is_empty(self)
        }

        fn len(&self) -> usize {
            HashMap::len(self)
        }

        fn to_vec(&self) -> Vec<V>
        where
            V: Clone,
        {
            self.values().cloned().collect()
        }
    }

    impl<K, V> Memory for HashMap<K, V>
    where
        K: Eq + core::hash::Hash,
    {
        type Key = K;

        fn get(&self, index: &K) -> Option<&V> {
            HashMap::get(self, index)
        }

        fn get_mut(&mut self, index: &K) -> Option<&mut V> {
            HashMap::get_mut(self, index)
        }
    }
}
