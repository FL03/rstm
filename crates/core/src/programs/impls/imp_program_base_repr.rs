/*
    Appellation: impl_program <module>
    Created At: 2026.01.11:12:33:32
    Contrib: @FL03
*/
use crate::programs::ProgramBase;
use crate::rules::Instruction;
use rstm_state::RawState;

impl<I, Q, A> ProgramBase<[I], Q, A, I>
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, I> {
        self.rules.iter()
    }
    /// returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, I> {
        self.rules.iter_mut()
    }
}

impl<I, Q, A> ProgramBase<&[I], Q, A, I>
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, I> {
        self.rules.iter()
    }
}

impl<I, Q, A> ProgramBase<&mut [I], Q, A, I>
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, I> {
        self.rules.iter()
    }
    /// returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, I> {
        self.rules.iter_mut()
    }
}

impl<const N: usize, I, Q, A> ProgramBase<[I; N], Q, A>
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, I> {
        self.rules.iter()
    }
    /// returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, I> {
        self.rules.iter_mut()
    }
}

#[cfg(feature = "alloc")]
impl<I, Q, A> ProgramBase<alloc::vec::Vec<I>, Q, A, I>
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, I> {
        self.rules.iter()
    }
    /// returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, I> {
        self.rules.iter_mut()
    }
}
