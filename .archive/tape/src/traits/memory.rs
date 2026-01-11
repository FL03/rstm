/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// [RawMemory] is a trait that provides a common interface for memory storage.
pub trait RawMemory {
    type Elem;

    private!();

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;
}
/// The [`Memory`] trait extends the base [`RawMemory`] trait to provide additional
/// functionality focused on accessing stored elements using some key.
pub trait Memory: RawMemory {
    type Key;

    fn get(&self, index: &Self::Key) -> Option<&Self::Elem>;
}

pub trait MemoryMut: Memory {
    fn get_mut(&mut self, index: &Self::Key) -> Option<&mut Self::Elem>;

    fn clear(&mut self);

    fn insert(&mut self, index: Self::Key, elem: Self::Elem);

    fn remove(&mut self, index: Self::Key) -> Option<Self::Elem>;
}

/// [`SeqMemory`] extends the base trait [`RawMemory`] to provide sequential access to memory.
pub trait SeqMemory: Memory {
    fn as_ptr(&self) -> *const Self::Elem;

    fn as_mut_ptr(&mut self) -> *mut Self::Elem;

    fn as_slice(&self) -> &[Self::Elem];

    fn as_mut_slice(&mut self) -> &mut [Self::Elem];

    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<Self::Elem>
    where
        Self::Elem: Clone;
}

/*
 ************* Implementations *************
*/

impl<T> RawMemory for &T
where
    T: RawMemory,
{
    type Elem = T::Elem;

    seal! {}

    fn is_empty(&self) -> bool {
        <T>::is_empty(self)
    }

    fn len(&self) -> usize {
        <T>::len(self)
    }
}

impl<T> RawMemory for &mut T
where
    T: RawMemory,
{
    type Elem = T::Elem;

    seal! {}

    fn is_empty(&self) -> bool {
        <T>::is_empty(self)
    }

    fn len(&self) -> usize {
        <T>::len(self)
    }
}

impl<T> RawMemory for [T] {
    type Elem = T;

    seal!();

    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }

    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T> Memory for [T] {
    type Key = usize;

    fn get(&self, index: &Self::Key) -> Option<&Self::Elem> {
        <[T]>::get(self, *index)
    }
}

impl<T> MemoryMut for [T]
where
    T: Clone,
{
    fn clear(&mut self) {
        self.fill(unsafe { core::mem::zeroed() });
    }
    fn insert(&mut self, index: usize, elem: Self::Elem) {
        self[index] = elem;
    }
    fn remove(&mut self, index: usize) -> Option<Self::Elem> {
        if index < self.len() {
            Some(core::mem::replace(&mut self[index], unsafe {
                core::mem::zeroed()
            }))
        } else {
            None
        }
    }
    fn get_mut(&mut self, index: &Self::Key) -> Option<&mut Self::Elem> {
        <[T]>::get_mut(self, *index)
    }
}

impl<T> SeqMemory for [T] {
    fn as_ptr(&self) -> *const T {
        <[T]>::as_ptr(self)
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        <[T]>::as_mut_ptr(self)
    }

    fn as_slice(&self) -> &[T] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        <[T]>::to_vec(self)
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{Memory, MemoryMut, RawMemory, SeqMemory};
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
    }

    impl<T> Memory for Vec<T> {
        type Key = usize;

        fn get(&self, index: &Self::Key) -> Option<&Self::Elem> {
            <[T]>::get(self, *index)
        }
    }
    impl<T> MemoryMut for Vec<T>
    where
        T: Clone,
    {
        fn clear(&mut self) {
            Vec::clear(self);
        }
        fn insert(&mut self, index: usize, elem: Self::Elem) {
            if index >= self.len() {
                self.push(elem);
            } else {
                self[index] = elem;
            }
        }
        fn remove(&mut self, index: usize) -> Option<Self::Elem> {
            if index < self.len() {
                Some(Vec::remove(self, index))
            } else {
                None
            }
        }
        fn get_mut(&mut self, index: &Self::Key) -> Option<&mut Self::Elem> {
            <[T]>::get_mut(self, *index)
        }
    }

    impl<T> SeqMemory for Vec<T>
    where
        Vec<T>: AsRef<[T]>,
    {
        fn as_slice(&self) -> &[T] {
            Vec::as_slice(self)
        }

        fn as_mut_slice(&mut self) -> &mut [T] {
            Vec::as_mut_slice(self)
        }
        fn to_vec(&self) -> Vec<T>
        where
            T: Clone,
        {
            self.clone()
        }

        fn as_ptr(&self) -> *const T {
            Vec::as_ptr(self)
        }

        fn as_mut_ptr(&mut self) -> *mut T {
            Vec::as_mut_ptr(self)
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::RawMemory;
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
    }
}
