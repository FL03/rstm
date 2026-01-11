/*
    Appellation: instruction_set <module>
    Created At: 2026.01.11:12:29:41
    Contrib: @FL03
*/
use crate::{RawHead, RawTail, State};
/// The [`Instruction`] trait establishes the base interface for all compatible rules for the
/// automata.
pub trait Instruction<Q, A> {
    /// the type of head used by the instruction
    type Head<_Q, _A>: RawHead<_Q, _A>;
    /// the type of tail used by the instruction
    type Tail<_Q, _A>: RawTail<_Q, _A>;
    /// returns a reference to the head of the instruction
    fn head(&self) -> &Self::Head<Q, A>;
    /// returns a reference to the tail of the instruction
    fn tail(&self) -> &Self::Tail<Q, A>;
    /// returns a reference to the current state
    fn current_state<'a>(&'a self) -> &'a State<Q>
    where
        Self::Head<Q, A>: 'a,
    {
        self.head().state()
    }
    /// returns a reference to the current symbol
    fn symbol<'a>(&'a self) -> &'a A
    where
        Self::Head<Q, A>: 'a,
    {
        self.head().symbol()
    }
    /// returns the direction of the tail
    fn direction(&self) -> crate::Direction {
        self.tail().direction()
    }
    /// returns a reference to the next state
    fn next_state<'a>(&'a self) -> &'a State<Q>
    where
        Self::Tail<Q, A>: 'a,
    {
        self.tail().next_state()
    }
    /// returns a reference to the next symbol
    fn next_symbol<'a>(&'a self) -> &'a A
    where
        Self::Tail<Q, A>: 'a,
    {
        self.tail().write_symbol()
    }
}

pub trait RuleSet<Q, A> {
    type Rule: Instruction<Q, A>;
}

/*
 ************* Implementations *************
*/
use crate::{Head, Rule, Tail};

impl<Q, A> Instruction<Q, A> for (Head<Q, A>, Tail<Q, A>) {
    type Head<_Q, _A> = Head<_Q, _A>;
    type Tail<_Q, _A> = Tail<_Q, _A>;

    fn head(&self) -> &Self::Head<Q, A> {
        &self.0
    }

    fn tail(&self) -> &Self::Tail<Q, A> {
        &self.1
    }
}

impl<Q, A> Instruction<Q, A> for Rule<Q, A> {
    type Head<_Q, _A> = Head<_Q, _A>;
    type Tail<_Q, _A> = Tail<_Q, _A>;

    fn head(&self) -> &Self::Head<Q, A> {
        &self.head
    }

    fn tail(&self) -> &Self::Tail<Q, A> {
        &self.tail
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::RuleSet;
    use crate::{Head, Rule, Tail};
    use alloc::collections::{BTreeMap, BTreeSet};
    use alloc::vec::Vec;

    impl<Q, A> RuleSet<Q, A> for Vec<(Head<Q, A>, Tail<Q, A>)> {
        type Rule = Rule<Q, A>;
    }

    impl<Q, A> RuleSet<Q, A> for Vec<Rule<Q, A>> {
        type Rule = Rule<Q, A>;
    }

    impl<Q, A> RuleSet<Q, A> for BTreeSet<Rule<Q, A>> {
        type Rule = Rule<Q, A>;
    }

    impl<Q, A> RuleSet<Q, A> for BTreeMap<Head<Q, A>, Tail<Q, A>> {
        type Rule = Rule<Q, A>;
    }
}

#[cfg(feature = "hashbrown")]
impl<Q, A> RuleSet<Q, A> for hashbrown::HashSet<Rule<Q, A>> {
    type Rule = Rule<Q, A>;
}

#[cfg(feature = "hashbrown")]
impl<Q, A> RuleSet<Q, A> for hashbrown::HashMap<Head<Q, A>, Tail<Q, A>> {
    type Rule = Rule<Q, A>;
}

#[cfg(feature = "std")]
impl<Q, A> RuleSet<Q, A> for std::collections::HashSet<Rule<Q, A>> {
    type Rule = Rule<Q, A>;
}

#[cfg(feature = "std")]
impl<Q, A> RuleSet<Q, A> for std::collections::HashMap<Head<Q, A>, Tail<Q, A>> {
    type Rule = Rule<Q, A>;
}
