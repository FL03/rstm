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

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>>;
}

pub trait RuleSetMut<Q, A>: RuleSet<Q, A>
where
    Q: RawState,
{
    fn get_mut(&mut self, head: &Head<Q, A>) -> Option<&mut Tail<Q, A>>;
}
/*
 ************* Implementations *************
*/

macro_rules! get_tail {
    ($iter:expr, $head:expr) => {
        $iter.find_map(|i| {
            if i.head() == $head {
                Some(i.tail())
            } else {
                None
            }
        })
    };
}

impl<Q, A> RuleSet<Q, A> for [(Head<Q, A>, Tail<Q, A>)]
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    type Rule = (Head<Q, A>, Tail<Q, A>);

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        self.iter()
            .find_map(|(h, t)| if h == head { Some(t) } else { None })
    }
}

impl<Q, A> RuleSet<Q, A> for [Rule<Q, A>]
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    type Rule = Rule<Q, A>;

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }
}

impl<Q, A> RuleSet<Q, A> for &[Rule<Q, A>]
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    type Rule = Rule<Q, A>;

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }
}

impl<Q, A> RuleSet<Q, A> for &mut [Rule<Q, A>]
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    type Rule = Rule<Q, A>;

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }
}

impl<const N: usize, Q, A> RuleSet<Q, A> for [Rule<Q, A>; N]
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    type Rule = Rule<Q, A>;

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }
}

impl<Q, A> RuleSetMut<Q, A> for &mut [Rule<Q, A>]
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    fn get_mut(&mut self, head: &Head<Q, A>) -> Option<&mut Tail<Q, A>> {
        self.iter_mut().find_map(|i| {
            if i.head() == head {
                Some(i.tail_mut())
            } else {
                None
            }
        })
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
        Q: RawState + PartialEq,
        A: PartialEq,
    {
        type Rule = Rule<Q, A>;
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            self.iter()
                .find_map(|(h, t)| if h == head { Some(t) } else { None })
        }
    }

    impl<Q, A> RuleSet<Q, A> for Vec<Rule<Q, A>>
    where
        Q: RawState + PartialEq,
        A: PartialEq,
    {
        type Rule = Rule<Q, A>;

        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            get_tail!(self.iter(), head)
        }
    }

    impl<Q, A> RuleSet<Q, A> for BTreeSet<Rule<Q, A>>
    where
        Q: RawState + PartialEq,
        A: PartialEq,
    {
        type Rule = Rule<Q, A>;
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            get_tail!(self.iter(), head)
        }
    }

    impl<Q, A> RuleSet<Q, A> for BTreeMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState + Ord,
        A: Ord,
    {
        type Rule = Rule<Q, A>;

        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            self.get(head)
        }
    }
}

#[cfg(feature = "hashbrown")]
impl<Q, A> RuleSet<Q, A> for hashbrown::HashSet<Rule<Q, A>>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    type Rule = Rule<Q, A>;

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }
}

#[cfg(feature = "hashbrown")]
impl<Q, A> RuleSet<Q, A> for hashbrown::HashMap<Head<Q, A>, Tail<Q, A>>
where
    Q: RawState + Eq + core::hash::Hash,
    A: Eq + core::hash::Hash,
{
    type Rule = Rule<Q, A>;

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        self.get(head)
    }
}

#[cfg(feature = "std")]
impl<Q, A> RuleSet<Q, A> for std::collections::HashSet<Rule<Q, A>>
where
    Q: RawState + Eq + core::hash::Hash,
    A: Eq + core::hash::Hash,
{
    type Rule = Rule<Q, A>;

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }
}

#[cfg(feature = "std")]
impl<Q, A> RuleSet<Q, A> for std::collections::HashMap<Head<Q, A>, Tail<Q, A>>
where
    Q: RawState + Eq + core::hash::Hash,
    A: Eq + core::hash::Hash,
{
    type Rule = Rule<Q, A>;

    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        self.get(head)
    }
}
