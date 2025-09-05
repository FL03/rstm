/*
    Appellation: impl_head_ext <module>
    Created At: 2025.09.05:17:57:25
    Contrib: @FL03
*/
use crate::head::Head;

use crate::Rule;
use crate::tail::Tail;

use rstm_state::{RawState, State};

impl<Q, S> core::fmt::Debug for Head<Q, S>
where
    Q: core::fmt::Debug,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Head")
            .field("state", &self.state)
            .field("symbol", &self.symbol)
            .finish()
    }
}

impl<Q, S> core::fmt::Display for Head<Q, S>
where
    Q: core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ state: {}, symbol: {} }}", self.state, self.symbol)
    }
}

/*
 ************* Operations *************
*/

impl<Q, A> core::ops::Add<Tail<Q, A>> for Head<Q, A>
where
    Q: RawState,
{
    type Output = Rule<Q, A>;

    fn add(self, rhs: Tail<Q, A>) -> Self::Output {
        Rule {
            head: self,
            tail: rhs,
        }
    }
}

/*
 ************* Equivalence *************
*/

impl<Q, S> PartialEq<State<Q>> for Head<Q, S>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, state: &State<Q>) -> bool {
        self.state() == state
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for State<Q>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        self == head.state()
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for State<&Q>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        *self == head.state().view()
    }
}

impl<'a, Q, S> PartialEq<State<&'a Q>> for Head<Q, S>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, state: &State<&'a Q>) -> bool {
        self.state().view() == *state
    }
}

impl<Q, S> PartialEq<(State<Q>, S)> for Head<Q, S>
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, (state, symbol): &(State<Q>, S)) -> bool {
        &self.state == state && &self.symbol == symbol
    }
}

impl<Q, S> PartialEq<(Q, S)> for Head<Q, S>
where
    State<Q>: PartialEq,
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, (state, symbol): &(Q, S)) -> bool {
        self.state() == state && self.symbol() == symbol
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for (State<Q>, S)
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        head.state() == &self.0 && head.symbol() == &self.1
    }
}
