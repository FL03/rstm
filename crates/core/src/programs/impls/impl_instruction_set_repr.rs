/*
    Appellation: impl_program <module>
    Created At: 2026.01.11:12:33:32
    Contrib: @FL03
*/
use crate::programs::ProgramBase;
use crate::rules::Rule;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use rstm_state::RawState;

impl<Q, A> ProgramBase<[Rule<Q, A>], Q, A>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, Rule<Q, A>> {
        self.rules.iter()
    }
    /// returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Rule<Q, A>> {
        self.rules.iter_mut()
    }
}

impl<Q, A> ProgramBase<&[Rule<Q, A>], Q, A>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, Rule<Q, A>> {
        self.rules.iter()
    }
}

impl<Q, A> ProgramBase<&mut [Rule<Q, A>], Q, A>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, Rule<Q, A>> {
        self.rules.iter()
    }
    /// returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Rule<Q, A>> {
        self.rules.iter_mut()
    }
}

impl<const N: usize, Q, A> ProgramBase<[Rule<Q, A>; N], Q, A>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, Rule<Q, A>> {
        self.rules.iter()
    }
    /// returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Rule<Q, A>> {
        self.rules.iter_mut()
    }
}

#[cfg(feature = "alloc")]
impl<Q, A> ProgramBase<Vec<Rule<Q, A>>, Q, A>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    /// returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, Rule<Q, A>> {
        self.rules.iter()
    }
    /// returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Rule<Q, A>> {
        self.rules.iter_mut()
    }
}
