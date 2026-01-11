/*
    Appellation: impl_store <module>
    Created At: 2025.08.30:17:54:40
    Contrib: @FL03
*/
use super::StoreBase;
use crate::{RawMemory, SeqMemory};
use rstm_core::Direction;

impl<A, S> StoreBase<S>
where
    S: SeqMemory<Elem = A>,
{
    pub fn new(tape: S) -> Self {
        let ptr = tape.as_ptr();
        let epoch = 0;
        Self { ptr, tape, epoch }
    }
    /// retusn an immutable reference to the tape
    pub const fn tape(&self) -> &S {
        &self.tape
    }
    /// returns a mutable reference to the current tape
    pub const fn tape_mut(&mut self) -> &mut S {
        &mut self.tape
    }
    /// returns the current epoch (or step) of the system
    pub const fn epoch(&self) -> usize {
        self.epoch
    }
    /// returns a mutable reference to the current epoch (or step) of the system
    pub const fn epoch_mut(&mut self) -> &mut usize {
        &mut self.epoch
    }
    /// returns the current pointer of the store
    pub const fn ptr(&self) -> *const A {
        self.ptr
    }
    /// returns an immutable reference to the current symbol under the tape head; `None` if the
    /// index is out of bounds
    pub fn read(&self) -> Option<&A> {
        unsafe { self.ptr.as_ref() }
    }
    /// applies the given direction to shift the tape head accordingly
    pub fn shift(self, direction: Direction) -> Self {
        unsafe {
            Self {
                ptr: self.ptr.offset(direction as isize),
                ..self
            }
        }
    }
    /// writes the given symbol to the current position of the tape head;
    ///
    /// ## Safety
    ///
    /// It is the caller's responsibility to ensure that the pointer is valid and properly
    /// aligned. Writing to an invalid or misaligned pointer can lead to undefined behavior.
    pub fn write(&mut self, symbol: A) {
        unsafe {
            self.tape_mut().as_mut_ptr().write(symbol);
        }
    }
}

#[allow(dead_code)]
impl<A, S> StoreBase<S>
where
    S: RawMemory<Elem = A>,
{
    /// a private method for incrementing the epoch counter whenever the tape is updated
    pub(crate) fn on_update(&mut self) {
        self.epoch += 1;
    }
}
