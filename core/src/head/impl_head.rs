/*
    Appellation: impl_head <module>
    Created At: 2025.08.31:00:00:55
    Contrib: @FL03
*/
use super::Head;
use rstm_state::{RawState, State};

impl<Q, S> Head<Q, S> where Q: RawState {}

impl<Q, S> core::fmt::Debug for Head<Q, S>
where
    Q: RawState,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Head")
            .field(&self.state)
            .field(&self.symbol)
            .finish()
    }
}

impl<Q, S> core::fmt::Display for Head<Q, S>
where
    Q: RawState,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ state: {:?}, symbol: {} }}", self.state, self.symbol)
    }
}

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

impl<Q, S> From<(Q, S)> for Head<Q, S>
where
    Q: RawState,
{
    fn from((state, symbol): (Q, S)) -> Self {
        Self::new(state, symbol)
    }
}

impl<Q, S> From<(State<Q>, S)> for Head<Q, S>
where
    Q: RawState,
{
    fn from((State(state), symbol): (State<Q>, S)) -> Self {
        Self::new(state, symbol)
    }
}

impl<Q, S> From<Head<Q, S>> for (State<Q>, S)
where
    Q: RawState,
{
    fn from(head: Head<Q, S>) -> Self {
        head.into_tuple()
    }
}
