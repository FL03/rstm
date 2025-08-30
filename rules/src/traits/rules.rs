/*
    Appellation: rules <module>
    Created At: 2025.08.30:08:09:36
    Contrib: @FL03
*/
use crate::rule::Rule;
use rstm_core::{Head, Tail};
use rstm_state::{RawState, State};
/// The [`Ruleset`] trait defines the core behaviors of a collection of rules
pub trait Ruleset<Q, S>
where
    Q: RawState,
{
    fn iter(&self) -> core::slice::Iter<'_, Rule<Q, S>>;
    /// returns the [`Tail`] associated with the given [`Head`], if it exists
    fn get_by_head(&self, head: Head<&Q, &S>) -> Option<Tail<&Q, &S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head_view() == head {
                Some(i.tail_view())
            } else {
                None
            }
        })
    }
    /// filters the contents of the ruleset by a given state
    fn filter_by_state(&self, state: State<&Q>) -> Vec<&Rule<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().filter(|i| *i.head() == state).collect()
    }
}
