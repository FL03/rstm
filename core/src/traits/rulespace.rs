/*
    Appellation: rulespace <module>
    Created At: 2025.12.16:17:22:50
    Contrib: @FL03
*/
use crate::head::Head;
use crate::rule::Rule;
use crate::state::State;
use rspace_traits::RawSpace;

/// A [`HeadSpace`] is a particular _kind_ of space that deals with the _heads_ of rules.
pub trait HeadSpace<Q, S>: RawSpace<Elem = Head<Q, S>> {}
/// If a _headspace_, or configuration space, defines all possible configurations of a system,
/// then a rulespace defines how those configurations can change or evolve over time. In other
/// words, if a c-space is a disconnected point cloud, then a rulespace is a set of rules that
/// connect those points together, defining morphisms between them.
pub trait RuleSpace<Q, S> {
    type Space<_Q, _S>: ?Sized + HeadSpace<_Q, _S>;

    /// Retrieve the underlying configuration space associated with this rulespace.
    fn get_rule(&self, state: &Q, symbol: &S) -> Option<&Rule<Q, S>>;
}

/*
 ************* Implementations *************
*/

impl<Q, S> HeadSpace<Q, S> for [Head<Q, S>] {}

impl<Q, S> RuleSpace<Q, S> for [Rule<Q, S>]
where
    Q: PartialEq,
    S: PartialEq,
{
    type Space<_Q, _S> = dyn HeadSpace<_Q, _S>;

    fn get_rule(&self, state: &Q, symbol: &S) -> Option<&Rule<Q, S>> {
        self.iter().find(|&Rule { head, .. }| {
            head.state().view() == State(state) && head.symbol() == symbol
        })
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::vec::Vec;

    impl<Q, A> HeadSpace<Q, A> for Vec<Head<Q, A>> {}

    impl<Q, A> RuleSpace<Q, A> for Vec<Rule<Q, A>>
    where
        Q: PartialEq,
        A: PartialEq,
    {
        type Space<_Q, _S> = dyn HeadSpace<_Q, _S>;

        fn get_rule(&self, state: &Q, symbol: &A) -> Option<&Rule<Q, A>> {
            self.iter().find(|&Rule { head, .. }| {
                head.state().view() == State(state) && head.symbol() == symbol
            })
        }
    }
}
