/*
    Appellation: instruction_set <module>
    Created At: 2026.01.11:12:29:41
    Contrib: @FL03
*/
use crate::{RawHead, RawTail};
use rstm_state::{RawState, State};
/// The [`Instruction`] trait establishes the base interface for all compatible rules for the
/// automata.
pub trait Instruction<Q, A>
where
    Q: RawState,
{
    /// the type of head used by the instruction
    type Head: RawHead<Q, A>;
    /// the type of tail used by the instruction
    type Tail: RawTail<Q, A>;
    /// returns a reference to the head of the instruction
    fn head(&self) -> &Self::Head;
    /// returns a reference to the tail of the instruction
    fn tail(&self) -> &Self::Tail;
    /// returns a reference to the current state
    fn current_state<'a>(&'a self) -> &'a State<Q>
    where
        Self::Head: 'a,
    {
        self.head().state()
    }
    /// returns a reference to the current symbol
    fn symbol<'a>(&'a self) -> &'a A
    where
        Self::Head: 'a,
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
        Self::Tail: 'a,
    {
        self.tail().next_state()
    }
    /// returns a reference to the next symbol
    fn next_symbol<'a>(&'a self) -> &'a A
    where
        Self::Tail: 'a,
    {
        self.tail().write_symbol()
    }
}

pub trait RuleSet<Q, A>
where
    Q: RawState,
{
    type Rule: Instruction<Q, A>;
}

/*
 ************* Implementations *************
*/
use crate::{Head, Rule, Tail};

impl<Q, A> Instruction<Q, A> for (Head<Q, A>, Tail<Q, A>)
where
    Q: RawState,
{
    type Head = Head<Q, A>;
    type Tail = Tail<Q, A>;

    fn head(&self) -> &Self::Head {
        &self.0
    }

    fn tail(&self) -> &Self::Tail {
        &self.1
    }
}

impl<Q, A> Instruction<Q, A> for Rule<Q, A>
where
    Q: RawState,
{
    type Head = Head<Q, A>;
    type Tail = Tail<Q, A>;

    fn head(&self) -> &Self::Head {
        &self.head
    }

    fn tail(&self) -> &Self::Tail {
        &self.tail
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::RuleSet;
    use crate::{Head, Rule, Tail};
    use alloc::collections::{BTreeMap, BTreeSet};
    use alloc::vec::Vec;
    use rstm_state::RawState;

    impl<Q, A> RuleSet<Q, A> for Vec<(Head<Q, A>, Tail<Q, A>)>
    where
        Q: RawState,
    {
        type Rule = Rule<Q, A>;
    }

    impl<Q, A> RuleSet<Q, A> for Vec<Rule<Q, A>>
    where
        Q: RawState,
    {
        type Rule = Rule<Q, A>;
    }

    impl<Q, A> RuleSet<Q, A> for BTreeSet<Rule<Q, A>>
    where
        Q: RawState,
    {
        type Rule = Rule<Q, A>;
    }

    impl<Q, A> RuleSet<Q, A> for BTreeMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState,
    {
        type Rule = Rule<Q, A>;
    }
}

#[cfg(feature = "hashbrown")]
impl<Q, A> RuleSet<Q, A> for hashbrown::HashSet<Rule<Q, A>>
where
    Q: RawState,
{
    type Rule = Rule<Q, A>;
}

#[cfg(feature = "hashbrown")]
impl<Q, A> RuleSet<Q, A> for hashbrown::HashMap<Head<Q, A>, Tail<Q, A>>
where
    Q: RawState,
{
    type Rule = Rule<Q, A>;
}

#[cfg(feature = "std")]
impl<Q, A> RuleSet<Q, A> for std::collections::HashSet<Rule<Q, A>>
where
    Q: RawState,
{
    type Rule = Rule<Q, A>;
}

#[cfg(feature = "std")]
impl<Q, A> RuleSet<Q, A> for std::collections::HashMap<Head<Q, A>, Tail<Q, A>>
where
    Q: RawState,
{
    type Rule = Rule<Q, A>;
}
