/*
    Appellation: store <module>
    Created At: 2025.08.30:17:54:58
    Contrib: @FL03
*/
mod impl_store;

use super::RawMemory;

pub type Store<A> = StoreBase<Vec<A>>;
/// The [`StoreBase`] struct provides an optimized sequential memory layout for a Turing machine
pub struct StoreBase<S>
where
    S: RawMemory,
{
    pub(crate) ptr: *const S::Elem,
    pub(crate) tape: S,
    pub(crate) epoch: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstm_core::Direction;

    #[test]
    fn test_shift_store() {
        let mut store = StoreBase::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(store.read(), Some(&1));
        store = store.shift(Direction::Right);
        assert_eq!(store.read(), Some(&2));
        store = store.shift(Direction::Right);
        assert_eq!(store.read(), Some(&3));
        store = store.shift(Direction::Left);
        assert_eq!(store.read(), Some(&2));
    }
}
