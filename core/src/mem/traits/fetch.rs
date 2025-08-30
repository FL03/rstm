/*
    appellation: fetch <module>
    authors: @FL03
*/
/// returns a reference to an element at a given index in a collection.
pub trait Fetch<K> {
    type Output;

    fn fetch(&self, index: K) -> Option<&Self::Output>;
}
/// returns a mutable reference to an element at a given index in a collection.
pub trait FetchMut<K>: Fetch<K> {
    /// Fetch a mutable reference to the element at the given index.
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
}

impl<I, T> FetchMut<I> for [T]
where
    I: core::slice::SliceIndex<[T]>,
    I::Output: Sized,
{
    fn fetch_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        <[T]>::get_mut(self, index)
    }
}
