/*
    Appellation: instruction_set <module>
    Created At: 2026.01.11:12:29:41
    Contrib: @FL03
*/
use crate::rules::{Head, Instruction, Rule, Tail};
use rstm_state::RawState;

/// The [`RuleSet`] trait establishes an interface common to all compatible sets of rules for 
/// the framework.
pub trait RuleSet<Q, A>
where
    Q: RawState,
{
    type Rule: Instruction<Q, A>;
}

/*
    ************* Implementations *************
*/

impl<Q, A> RuleSet<Q, A> for [(Head<Q, A>, Tail<Q, A>)]
where
    Q: RawState,
{
    type Rule = (Head<Q, A>, Tail<Q, A>);
}

impl<Q, A> RuleSet<Q, A> for [Rule<Q, A>]
where
    Q: RawState,
{
    type Rule = Rule<Q, A>;
}

impl<Q, A> RuleSet<Q, A> for &[Rule<Q, A>]
where
    Q: RawState,
{
    type Rule = Rule<Q, A>;
}

impl<Q, A> RuleSet<Q, A> for &mut [Rule<Q, A>]
where
    Q: RawState,
{
    type Rule = Rule<Q, A>;
}

impl<const N: usize, Q, A> RuleSet<Q, A> for [Rule<Q, A>; N]
where
    Q: RawState,
{
    type Rule = Rule<Q, A>;
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
