/*
    Appellation: instruction_set <module>
    Created At: 2026.01.11:12:29:41
    Contrib: @FL03
*/
use crate::rules::{Head, Instruction, InstructionMut, Rule, Tail};
use rstm_state::{RawState, State};

/// The [`RawRuleset`] trait establishes an interface common to all compatible sets of rules for
/// the framework.
pub trait RawRuleset<Q, A>
where
    Q: RawState,
{
    type Rule: Instruction<Q, A>;

    private! {}

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait Ruleset<Q, A>: RawRuleset<Q, A>
where
    Q: RawState,
    Self::Rule: Instruction<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
{

    fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>>;
    
    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>>;

    fn find_head(&self, Head { state, symbol }: Head<&Q, &A>) -> Option<&Tail<Q, A>> {
        self.find_tail(state, symbol)
    }
}

pub trait RuleSetMut<Q, A>: RawRuleset<Q, A>
where
    Q: RawState,
    Self::Rule: Instruction<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
{
    fn get_mut(&mut self, head: &Head<Q, A>) -> Option<&mut Tail<Q, A>>;
}
/*
 ************* Implementations *************
*/

impl<R, I, Q, A> RawRuleset<Q, A> for &R
where
    I: Instruction<Q, A>,
    Q: RawState,
    R: RawRuleset<Q, A, Rule = I>,
{
    type Rule = R::Rule;

    seal! {}

    fn len(&self) -> usize {
        (*self).len()
    }
}

macro_rules! get_tail {
    ($iter:expr, $head:expr) => {
        $iter.find_map(|i| {
            if i.head().state() == $head.state() && i.head().symbol() == $head.symbol() {
                Some(i.tail())
            } else {
                None
            }
        })
    };
}

macro_rules! find_tail {
    ($iter:expr, ($state:expr, $sym:expr)) => {
        $iter.find_map(|i| {
            if i.head().state().view() == $state && i.head().symbol() == $sym {
                Some(i.tail())
            } else {
                None
            }
        })
    };
}

impl<I, Q, A> RawRuleset<Q, A> for [I]
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    type Rule = I;

    seal! {}

    fn len(&self) -> usize {
        <[I]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[I]>::is_empty(self)
    }
}

impl<I, Q, A> Ruleset<Q, A> for [I]
where
    Q: RawState + PartialEq,
    A: PartialEq,
    I: Instruction<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
{
    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }

    fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
        find_tail!(self.iter(), (state, sym))
    }
}
impl<I, Q, A> RawRuleset<Q, A> for &[I]
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    type Rule = I;

    seal! {}

    fn len(&self) -> usize {
        <[I]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[I]>::is_empty(self)
    }
}

impl<I, Q, A> Ruleset<Q, A> for &[I]
where
    Q: RawState + PartialEq,
    A: PartialEq,
    I: Instruction<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
{
    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }

    fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
        find_tail!(self.iter(), (state, sym))
    }
}

impl<I, Q, A> RawRuleset<Q, A> for &mut [I]
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    type Rule = I;

    seal! {}

    fn len(&self) -> usize {
        <[I]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[I]>::is_empty(self)
    }
}

impl<I, Q, A> Ruleset<Q, A> for &mut [I]
where
    Q: RawState + PartialEq,
    A: PartialEq,
    I: Instruction<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
{
    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }

    fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
        find_tail!(self.iter(), (state, sym))
    }
}

impl<I, Q, A> RuleSetMut<Q, A> for &mut [I]
where
    Q: RawState + PartialEq,
    A: PartialEq,
    I: InstructionMut<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
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

impl<const N: usize, I, Q, A> RawRuleset<Q, A> for [I; N]
where
    Q: RawState,
    I: Instruction<Q, A>,
{
    type Rule = I;

    seal! {}

    fn len(&self) -> usize {
        <[I]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[I]>::is_empty(self)
    }
}

impl<const N: usize, I, Q, A> Ruleset<Q, A> for [I; N]
where
    Q: RawState + PartialEq,
    A: PartialEq,
    I: Instruction<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
{
    fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        get_tail!(self.iter(), head)
    }

    fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
        find_tail!(self.iter(), (state, sym))
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{RawRuleset, Ruleset};
    use crate::{Head, Instruction, Rule, Tail};
    use alloc::collections::{BTreeMap, BTreeSet};
    use alloc::vec::Vec;
    use rstm_state::{RawState, State};

    impl<I, Q, A> RawRuleset<Q, A> for Vec<I>
    where
        Q: RawState,
        I: Instruction<Q, A>,
    {
        type Rule = I;

        seal! {}

        fn len(&self) -> usize {
            <Vec<I>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <Vec<I>>::is_empty(self)
        }
    }

    impl<I, Q, A> Ruleset<Q, A> for Vec<I>
    where
        Q: RawState + PartialEq,
        A: PartialEq,
        I: Instruction<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
    {
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            get_tail!(self.iter(), head)
        }

        fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
            find_tail!(self.iter(), (state, sym))
        }
    }

    impl<I, Q, A> RawRuleset<Q, A> for BTreeSet<I>
    where
        Q: RawState,
        I: Instruction<Q, A>,
    {
        type Rule = I;

        seal! {}

        fn len(&self) -> usize {
            <BTreeSet<I>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <BTreeSet<I>>::is_empty(self)
        }
    }

    impl<I, Q, A> Ruleset<Q, A> for BTreeSet<I>
    where
        Q: RawState + PartialEq,
        A: PartialEq,
        I: Instruction<Q, A, Head = Head<Q, A>, Tail = Tail<Q, A>>,
    {
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            get_tail!(self.iter(), head)
        }

        fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
            find_tail!(self.iter(), (state, sym))
        }
    }

    impl<Q, A> RawRuleset<Q, A> for BTreeMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState + Ord,
        A: Ord,
    {
        type Rule = Rule<Q, A>;

        seal! {}

        fn len(&self) -> usize {
            <BTreeMap<Head<Q, A>, Tail<Q, A>>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <BTreeMap<Head<Q, A>, Tail<Q, A>>>::is_empty(self)
        }
    }

    impl<Q, A> Ruleset<Q, A> for BTreeMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState + Ord,
        A: Ord,
    {
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            self.get(head)
        }

        fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
            self.iter().find_map(|(h, t)| {
                if h.state().view() == state && h.symbol() == sym {
                    Some(t)
                } else {
                    None
                }
            })
        }
    }
}

#[cfg(feature = "hashbrown")]
mod impl_hashbrown {
    use super::*;
    use core::hash::Hash;
    use hashbrown::{HashMap, HashSet};

    impl<I, Q, A> RawRuleset<Q, A> for HashSet<I>
    where
        I: Instruction<Q, A>,
        Q: RawState + Eq + Hash,
        A: Eq + Hash,
    {
        type Rule = Rule<Q, A>;

        seal! {}

        fn len(&self) -> usize {
            <HashSet<I>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <HashSet<I>>::is_empty(self)
        }
    }

    impl<Q, A> Ruleset<Q, A> for HashSet<Rule<Q, A>>
    where
        Q: RawState + Eq + Hash,
        A: Eq + Hash,
    {
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            get_tail!(self.iter(), head)
        }

        fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
            find_tail!(self.iter(), (state, sym))
        }
    }

    impl<Q, A> RawRuleset<Q, A> for HashMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState + Eq + Hash,
        A: Eq + Hash,
    {
        type Rule = Rule<Q, A>;

        seal! {}

        fn len(&self) -> usize {
            <HashMap<Head<Q, A>, Tail<Q, A>>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <HashMap<Head<Q, A>, Tail<Q, A>>>::is_empty(self)
        }
    }

    impl<Q, A> Ruleset<Q, A> for HashMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState + Eq + Hash,
        A: Eq + Hash,
    {
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            self.get(head)
        }

        fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
            self.iter().find_map(|(h, t)| {
                if h.state().view() == state && h.symbol() == sym {
                    Some(t)
                } else {
                    None
                }
            })
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::*;
    use core::hash::Hash;
    use std::collections::{HashMap, HashSet};

    impl<I, Q, A> RawRuleset<Q, A> for HashSet<I>
    where
        I: Instruction<Q, A>,
        Q: RawState + Eq + Hash,
        A: Eq + Hash,
    {
        type Rule = Rule<Q, A>;

        seal! {}

        fn len(&self) -> usize {
            <HashSet<I>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <HashSet<I>>::is_empty(self)
        }
    }

    impl<Q, A> Ruleset<Q, A> for HashSet<Rule<Q, A>>
    where
        Q: RawState + Eq + Hash,
        A: Eq + Hash,
    {
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            get_tail!(self.iter(), head)
        }

        fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
            find_tail!(self.iter(), (state, sym))
        }
    }

    impl<Q, A> RawRuleset<Q, A> for HashMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState + Eq + Hash,
        A: Eq + Hash,
    {
        type Rule = Rule<Q, A>;

        seal! {}

        fn len(&self) -> usize {
            <HashMap<Head<Q, A>, Tail<Q, A>>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <HashMap<Head<Q, A>, Tail<Q, A>>>::is_empty(self)
        }
    }

    impl<Q, A> Ruleset<Q, A> for HashMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState + Eq + Hash,
        A: Eq + Hash,
    {
        fn get(&self, head: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            self.get(head)
        }

        fn find_tail(&self, state: State<&Q>, sym: &A) -> Option<&Tail<Q, A>> {
            self.iter().find_map(|(h, t)| {
                if h.state() == *state && h.symbol() == sym {
                    Some(t)
                } else {
                    None
                }
            })
        }
    }
}
