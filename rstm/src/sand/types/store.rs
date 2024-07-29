/*
    Appellation: symbol <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{StdHead, StdTail, Symbolic, SymbolicExt};
use std::collections::hash_map::{self, Entry, HashMap};

pub struct Ruleset<Q, S = char> {
    pub(crate) rules: HashMap<StdHead<Q, S>, StdTail<Q, S>>,
}

impl<Q, S> Ruleset<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Symbolic,
{
    pub fn new() -> Self {
        Ruleset {
            rules: HashMap::new(),
        }
    }
    pub fn from_iter<I, R>(iter: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: Into<(StdHead<Q, S>, StdTail<Q, S>)>,
        HashMap<StdHead<Q, S>, StdTail<Q, S>>: FromIterator<I::Item>,
    {
        Ruleset {
            rules: iter.into_iter().collect(),
        }
    }
    pub fn from_rules(rules: HashMap<StdHead<Q, S>, StdTail<Q, S>>) -> Self {
        Ruleset { rules }
    }
    /// Given a head, returns an [entry](Entry) in the ruleset for in-place manipulation
    pub fn entry(&mut self, key: StdHead<Q, S>) -> Entry<'_, StdHead<Q, S>, StdTail<Q, S>>
    where
        S: SymbolicExt,
    {
        self.rules.entry(key)
    }
    /// Inserts a new rule into the ruleset
    pub fn insert(&mut self, head: StdHead<Q, S>, tail: StdTail<Q, S>)
    where
        S: SymbolicExt,
    {
        self.rules.insert(head, tail);
    }
    /// Returns a reference to the tail of a given head;
    /// if the head is not in the ruleset, returns [None](Option::None)
    pub fn get(&self, head: &StdHead<Q, S>) -> Option<&StdTail<Q, S>>
    where
        S: SymbolicExt,
    {
        self.rules.get(head)
    }
    /// Returns a mutable reference to the tail of a given head;
    /// if the head is not in the ruleset, returns [None](Option::None)
    pub fn get_mut(&mut self, head: &StdHead<Q, S>) -> Option<&mut StdTail<Q, S>>
    where
        S: SymbolicExt,
    {
        self.rules.get_mut(head)
    }
    /// Returns the number of rules in the ruleset
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Check to see whether the ruleset is empty
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn iter(&self) -> hash_map::Iter<'_, StdHead<Q, S>, StdTail<Q, S>> {
        self.rules.iter()
    }

    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, StdHead<Q, S>, StdTail<Q, S>> {
        self.rules.iter_mut()
    }

    pub fn keys(&self) -> hash_map::Keys<'_, StdHead<Q, S>, StdTail<Q, S>> {
        self.rules.keys()
    }

    pub fn values(&self) -> hash_map::Values<'_, StdHead<Q, S>, StdTail<Q, S>> {
        self.rules.values()
    }

    pub fn values_mut(&mut self) -> hash_map::ValuesMut<'_, StdHead<Q, S>, StdTail<Q, S>> {
        self.rules.values_mut()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&StdHead<Q, S>, &mut StdTail<Q, S>) -> bool,
    {
        self.rules.retain(f)
    }

    pub fn remove(&mut self, head: &StdHead<Q, S>) -> Option<StdTail<Q, S>>
    where
        S: SymbolicExt,
    {
        self.rules.remove(head)
    }
}

impl<Q, S> Extend<(StdHead<Q, S>, StdTail<Q, S>)> for Ruleset<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: SymbolicExt,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (StdHead<Q, S>, StdTail<Q, S>)>,
    {
        self.rules.extend(iter)
    }
}

impl<Q, S> Extend<crate::rules::Instruction<Q, S>> for Ruleset<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: SymbolicExt,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = crate::rules::Instruction<Q, S>>,
    {
        self.rules
            .extend(iter.into_iter().map(|i| (i.head, i.tail)))
    }
}

impl<Q, S> IntoIterator for Ruleset<Q, S> {
    type Item = (StdHead<Q, S>, StdTail<Q, S>);
    type IntoIter = std::collections::hash_map::IntoIter<StdHead<Q, S>, StdTail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}
