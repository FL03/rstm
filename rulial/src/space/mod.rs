/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Rulial Space
//!
//!
//! ## Definition
//!
//! Configuration space `C` is the set of all possible configurations `q` of a system where
//! `q in Q`.
#[doc(inline)]
pub use self::point::BasePoint;

pub(crate) mod point;

pub(crate) mod prelude {
    pub use super::point::*;
    pub use super::{RawPoint, RawSpace, Space};
}

/// The [`RawPoint`] trait describes a point in a space.
pub trait RawPoint {
    type Elem;

    fn as_ptr(&self) -> *const Self::Elem;

    fn as_mut_ptr(&mut self) -> *mut Self::Elem;

    fn as_slice(&self) -> &[Self::Elem];

    fn as_mut_slice(&mut self) -> &mut [Self::Elem];

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// [RawSpace] describes
pub unsafe trait RawSpace<T> {
    type Elem: RawPoint<Elem = T>;
}

/// [Space] describes a space.
pub trait Space<T>: RawSpace<T> {}

/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

impl<A> RawPoint for [A] {
    type Elem = A;

    fn as_ptr(&self) -> *const Self::Elem {
        self.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut Self::Elem {
        self.as_mut_ptr()
    }

    fn as_slice(&self) -> &[Self::Elem] {
        &self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "alloc")]
impl<T> RawPoint for Vec<T> {
    type Elem = T;

    fn as_ptr(&self) -> *const Self::Elem {
        Vec::as_ptr(self)
    }

    fn as_mut_ptr(&mut self) -> *mut Self::Elem {
        Vec::as_mut_ptr(self)
    }

    fn as_slice(&self) -> &[Self::Elem] {
        Vec::as_slice(self)
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        Vec::as_mut_slice(self)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<S> RawPoint for BasePoint<S>
where
    S: RawPoint,
{
    type Elem = S::Elem;

    fn as_ptr(&self) -> *const Self::Elem {
        self.data.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut Self::Elem {
        self.data.as_mut_ptr()
    }

    fn as_slice(&self) -> &[Self::Elem] {
        self.data.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self.data.as_mut_slice()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
