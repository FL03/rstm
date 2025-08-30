/*
    Appellation: container <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{RawMemory, SeqMemory};
use rstm_core::Direction;

pub type Store<A> = StoreBase<Vec<A>>;

pub struct StoreBase<S>
where
    S: RawMemory,
{
    pub(crate) ptr: *const S::Elem,
    pub(crate) tape: S,
    pub(crate) time: usize,
}

impl<A, S> StoreBase<S>
where
    S: SeqMemory<Elem = A>,
{
    pub fn new(tape: S) -> Self {
        let ptr = tape.as_ptr();
        let time = 0;
        Self { ptr, tape, time }
    }

    pub fn tape(&self) -> &S {
        &self.tape
    }

    pub fn tape_mut(&mut self) -> &mut S {
        &mut self.tape
    }

    pub fn time(&self) -> usize {
        self.time
    }

    pub fn time_mut(&mut self) -> &mut usize {
        &mut self.time
    }

    pub fn ptr(&self) -> *const A {
        self.ptr
    }

    pub fn read(&self) -> Option<&A> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn shift(self, direction: Direction) -> Self {
        unsafe {
            Self {
                ptr: self.ptr.offset(direction as isize),
                ..self
            }
        }
    }

    pub fn write(&mut self, symbol: A) {
        unsafe {
            self.tape_mut().as_mut_ptr().write(symbol);
        }
    }
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
