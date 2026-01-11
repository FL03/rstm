/*
    Appellation: impl_program <module>
    Created At: 2025.09.04:19:04:56
    Contrib: @FL03
*/
use crate::program::Program;

use crate::types::RuleVec;
use rstm_core::{Head, Rule, Tail};
use rstm_state::{RawState, State};

use alloc::vec::{self, Vec};

impl<Q, A> Program<Q, A>
where
    Q: RawState,
{
    /// returns an immutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head() == head {
                Some(i.tail())
            } else {
                None
            }
        })
    }
    /// returns a mutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn get_mut(&mut self, head: &Head<Q, A>) -> Option<&mut Tail<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
    {
        self.iter_mut().find_map(|i| {
            if i.head() == head {
                Some(i.tail_mut())
            } else {
                None
            }
        })
    }
    /// returns an immutable reference to the tail of a rule whose state and symbol match the
    /// givens
    pub fn find_tail(&self, state: State<&Q>, symbol: &A) -> Option<&Tail<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head().view() == (Head { state, symbol }) {
                Some(i.tail())
            } else {
                None
            }
        })
    }
    /// returns a mutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn find_mut_tail(&mut self, state: State<&Q>, symbol: &A) -> Option<&mut Tail<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
    {
        self.iter_mut().find_map(|i| {
            if i.head().view() == (Head { state, symbol }) {
                Some(i.tail_mut())
            } else {
                None
            }
        })
    }
    /// Returns a collection of rules whose head contains a match for the given state.
    pub fn filter_by_state(&self, state: State<&Q>) -> Vec<&Rule<Q, A>>
    where
        Q: PartialEq,
    {
        self.iter().filter(|i| *i.head() == state).collect()
    }
    #[cfg(all(feature = "json", feature = "std"))]
    /// saves the current program as a `.json` file at the given path
    pub fn export_json<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()>
    where
        Q: serde::Serialize,
        A: serde::Serialize,
    {
        let serialized = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, serialized)?;
        #[cfg(feature = "tracing")]
        tracing::info!("Program exported as JSON");
        Ok(())
    }
}

impl<Q, A> AsRef<[Rule<Q, A>]> for Program<Q, A>
where
    Q: RawState,
{
    fn as_ref(&self) -> &[Rule<Q, A>] {
        self.rules()
    }
}

impl<Q, A> AsMut<[Rule<Q, A>]> for Program<Q, A>
where
    Q: RawState,
{
    fn as_mut(&mut self) -> &mut [Rule<Q, A>] {
        self.rules_mut()
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
