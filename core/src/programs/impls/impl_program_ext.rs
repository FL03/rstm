/*
    Appellation: impl_program_ext <module>
    Created At: 2026.01.11:13:35:13
    Contrib: @FL03
*/
#![cfg(feature = "alloc")]
use crate::programs::{Program, RuleVec};

use crate::{Head, Rule, Tail};
use alloc::vec::{self, Vec};
use rstm_state::RawState;

impl<Q, A> AsRef<[Rule<Q, A>]> for Program<Q, A>
where
    Q: RawState,
{
    fn as_ref(&self) -> &[Rule<Q, A>] {
        &self.rules
    }
}

impl<Q, A> AsMut<[Rule<Q, A>]> for Program<Q, A>
where
    Q: RawState,
{
    fn as_mut(&mut self) -> &mut [Rule<Q, A>] {
        &mut self.rules
    }
}

impl<Q, A> core::ops::Deref for Program<Q, A>
where
    Q: RawState,
{
    type Target = [Rule<Q, A>];

    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}

impl<Q, A> core::ops::DerefMut for Program<Q, A>
where
    Q: RawState,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rules
    }
}

impl<Q, A> core::ops::Index<Head<Q, A>> for Program<Q, A>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    type Output = Tail<Q, A>;

    fn index(&self, index: Head<Q, A>) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<Q, A> From<Vec<Rule<Q, A>>> for Program<Q, A>
where
    Q: RawState + Default,
{
    fn from(rules: Vec<Rule<Q, A>>) -> Self {
        Self::from_rules(rules)
    }
}

impl<Q, A> Extend<Rule<Q, A>> for Program<Q, A>
where
    Q: RawState,
{
    fn extend<I: IntoIterator<Item = Rule<Q, A>>>(&mut self, iter: I) {
        self.rules_mut().extend(iter)
    }
}

impl<Q, A> Extend<(Head<Q, A>, Tail<Q, A>)> for Program<Q, A>
where
    Q: RawState,
{
    fn extend<I: IntoIterator<Item = (Head<Q, A>, Tail<Q, A>)>>(&mut self, iter: I) {
        self.rules_mut()
            .extend(iter.into_iter().map(|(head, tail)| Rule { head, tail }))
    }
}

impl<Q, A> FromIterator<Rule<Q, A>> for Program<Q, A>
where
    Q: RawState,
{
    fn from_iter<I: IntoIterator<Item = Rule<Q, A>>>(iter: I) -> Self {
        Self {
            initial_state: None,
            rules: iter.into_iter().collect::<RuleVec<Q, A>>(),
        }
    }
}

impl<Q, A> FromIterator<(Head<Q, A>, Tail<Q, A>)> for Program<Q, A>
where
    Q: RawState,
{
    fn from_iter<I: IntoIterator<Item = (Head<Q, A>, Tail<Q, A>)>>(iter: I) -> Self {
        Self::from_rules(iter.into_iter().map(|(head, tail)| Rule { head, tail }))
    }
}

impl<Q, A> IntoIterator for Program<Q, A>
where
    Q: RawState,
{
    type Item = Rule<Q, A>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}
