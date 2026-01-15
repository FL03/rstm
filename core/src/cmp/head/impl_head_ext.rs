/*
    Appellation: impl_head_ext <module>
    Created At: 2025.09.05:17:57:25
    Contrib: @FL03
*/
use super::Head;
use crate::rules::Rule;
use crate::tail::Tail;
use rstm_state::{RawState, State};

impl<Q, A> core::fmt::Debug for Head<Q, A>
where
    Q: core::fmt::Debug,
    A: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Head")
            .field(&self.state)
            .field(&self.symbol)
            .finish()
    }
}

impl<Q, A> core::fmt::Display for Head<Q, A>
where
    Q: core::fmt::Display,
    A: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write! { f, "{{ state: {}, symbol: {} }}", self.state, self.symbol }
    }
}

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

impl<Q, A> PartialEq<State<Q>> for Head<Q, A>
where
    Q: PartialEq,
{
    fn eq(&self, state: &State<Q>) -> bool {
        &self.state == state
    }
}

impl<Q, A> PartialEq<Head<Q, A>> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, head: &Head<Q, A>) -> bool {
        self == &head.state
    }
}

impl<Q, A> PartialEq<Head<Q, A>> for State<&Q>
where
    Q: PartialEq,
{
    fn eq(&self, head: &Head<Q, A>) -> bool {
        *self == head.state.view()
    }
}

impl<'a, Q, A> PartialEq<State<&'a Q>> for Head<Q, A>
where
    Q: PartialEq,
{
    fn eq(&self, state: &State<&'a Q>) -> bool {
        self.state.view() == *state
    }
}

impl<Q, A> PartialEq<(State<Q>, A)> for Head<Q, A>
where
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, (state, symbol): &(State<Q>, A)) -> bool {
        &self.state == state && &self.symbol == symbol
    }
}

impl<Q, A> PartialEq<Head<Q, A>> for (State<Q>, A)
where
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, head: &Head<Q, A>) -> bool {
        &self.0 == &head.state && &self.1 == &head.symbol
    }
}

impl<Q, A> PartialEq<(Q, A)> for Head<Q, A>
where
    State<Q>: PartialEq,
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, (state, symbol): &(Q, A)) -> bool {
        &self.state == state && &self.symbol == symbol
    }
}

impl<Q, A> PartialEq<Head<Q, A>> for (Q, A)
where
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, head: &Head<Q, A>) -> bool {
        head.state == &self.0 && &head.symbol == &self.1
    }
}

impl<Q, S> From<S> for Head<Q, S>
where
    Q: Default,
{
    fn from(symbol: S) -> Self {
        Head {
            state: State::default(),
            symbol,
        }
    }
}

impl<Q, S> From<(Q, S)> for Head<Q, S> {
    fn from((state, symbol): (Q, S)) -> Self {
        Head {
            state: State(state),
            symbol,
        }
    }
}

impl<Q, S> From<(State<Q>, S)> for Head<Q, S> {
    fn from((state, symbol): (State<Q>, S)) -> Self {
        Head { state, symbol }
    }
}

impl<Q, S> From<Head<Q, S>> for (State<Q>, S) {
    fn from(Head { state, symbol }: Head<Q, S>) -> Self {
        (state, symbol)
    }
}
