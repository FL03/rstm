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
}

pub trait Memory: RawMemory {
    fn to_vec(&self) -> Vec<Self::Elem>
    where
        Self::Elem: Clone;
}

pub trait MemoryMut: Memory {
    fn clear(&mut self);

    fn insert(&mut self, index: usize, elem: Self::Elem);

    fn remove(&mut self, index: usize) -> Option<Self::Elem>;
}

/// [Sequential] extends the base trait [RawMemory] to provide sequential access to memory.
pub trait SeqMemory: Memory {
    fn as_ptr(&self) -> *const Self::Elem;

    fn as_mut_ptr(&mut self) -> *mut Self::Elem;

    fn as_slice(&self) -> &[Self::Elem];

    fn as_mut_slice(&mut self) -> &mut [Self::Elem];

    fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[Self::Elem]>;

    fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: core::slice::SliceIndex<[Self::Elem]>;
}

#[doc(hidden)]
pub trait Fetch<K> {
    type Output;

    fn fetch(&self, index: K) -> Option<&Self::Output>;

    fn fetch_mut(&mut self, index: K) -> Option<&mut Self::Output>;
}

/*
 ************* Implementations *************
*/
impl<I, T> Fetch<I> for [T]
where
    I: core::slice::SliceIndex<[T]>,
    I::Output: Sized,
{
    type Output = I::Output;

    fn fetch(&self, index: I) -> Option<&Self::Output> {
        <[T]>::get(self, index)
    }

    fn fetch_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        <[T]>::get_mut(self, index)
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
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        <[T]>::to_vec(self)
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

    fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[T]>,
    {
        <[T]>::get(self, index)
    }

    fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: core::slice::SliceIndex<[T]>,
    {
        <[T]>::get_mut(self, index)
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use alloc::vec::Vec;

    impl<T> super::RawMemory for Vec<T> {
        type Elem = T;

        seal!();

        fn is_empty(&self) -> bool {
            Vec::is_empty(self)
        }

        fn len(&self) -> usize {
            Vec::len(self)
        }
    }

    impl<T> super::Memory for Vec<T> {
        fn to_vec(&self) -> Vec<T>
        where
            T: Clone,
        {
            self.clone()
        }
    }

    impl<T> super::SeqMemory for Vec<T>
    where
        Vec<T>: AsRef<[T]>,
    {
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

        fn get<I>(&self, index: I) -> Option<&I::Output>
        where
            I: core::slice::SliceIndex<[T]>,
        {
            <[T]>::get(self, index)
        }

        fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
        where
            I: core::slice::SliceIndex<[T]>,
        {
            <[T]>::get_mut(self, index)
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use std::collections::HashMap;

    impl<K, V> super::RawMemory for HashMap<K, V> {
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
