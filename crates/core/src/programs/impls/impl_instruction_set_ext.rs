/*
    Appellation: impl_program <module>
    Created At: 2026.01.11:12:33:32
    Contrib: @FL03
*/
use crate::programs::{ProgramBase, RawRuleset};
use crate::rules::Instruction;
use rstm_state::RawState;

impl<R, I, Q, A> core::fmt::Debug for ProgramBase<R, Q, A, I>
where
    R: RawRuleset<Q, A, Rule = I> + core::fmt::Debug,
    Q: RawState + core::fmt::Debug,
    I: Instruction<Q, A> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(initial_state) = &self.initial_state {
            write! {
                f,
                "{{ initial_state: {:?}, rules: {:?} }}",
                initial_state, self.rules
            }
        } else {
            write! { f, "{{ rules: {:?} }}", self.rules }
        }
    }
}

impl<R, I, Q, A> core::fmt::Display for ProgramBase<R, Q, A, I>
where
    R: RawRuleset<Q, A, Rule = I> + core::fmt::Debug,
    Q: RawState + core::fmt::Display,
    I: Instruction<Q, A> + core::fmt::Debug + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(initial_state) = &self.initial_state {
            write! {
                f,
                "{{ initial_state: {}, rules: {:?} }}",
                initial_state, self.rules
            }
        } else {
            write! { f, "{{ rules: {:?} }}", self.rules }
        }
    }
}

impl<I, X, R, Q, A> IntoIterator for ProgramBase<R, Q, A, X>
where
    I: Iterator<Item = X>,
    Q: RawState,
    X: Instruction<Q, A>,
    R: RawRuleset<Q, A, Rule = X> + IntoIterator<Item = X, IntoIter = I>,
{
    type Item = R::Item;
    type IntoIter = R::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}

impl<'a, X, I, R, Q, A> IntoIterator for &'a ProgramBase<R, Q, A, X>
where
    Q: RawState,
    X: 'a + Instruction<Q, A>,
    R: RawRuleset<Q, A, Rule = X>,
    I: Iterator<Item = &'a X>,
    for<'b> &'b R: IntoIterator<Item = &'b X, IntoIter = I>,
{
    type Item = &'a X;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}

impl<X, R, Q, A> FromIterator<X> for ProgramBase<R, Q, A, X>
where
    Q: RawState,
    X: Instruction<Q, A>,
    R: RawRuleset<Q, A, Rule = X> + FromIterator<X>,
{
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: IntoIterator<Item = X>,
    {
        Self::from_rules(<R>::from_iter(iter))
    }
}

impl<X, R, Q, A> Extend<X> for ProgramBase<R, Q, A, X>
where
    Q: RawState,
    X: Instruction<Q, A>,
    R: RawRuleset<Q, A, Rule = X> + Extend<X>,
{
    fn extend<Iter>(&mut self, iter: Iter)
    where
        Iter: IntoIterator<Item = X>,
    {
        self.rules.extend(iter)
    }
}
