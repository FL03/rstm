/*
    Appellation: impl_head_ext <module>
    Created At: 2025.09.05:17:57:25
    Contrib: @FL03
*/
use crate::rules::Head;
use crate::{Rule, Tail};
use rstm_state::{RawState, State};
use rstm_traits::{Read, ReadBuf};

impl<Q, A, T> ReadBuf<T> for Head<Q, A>
where
    Q: RawState,
    A: Copy + core::slice::SliceIndex<[T], Output = T>,
    A::Output: Clone,
{
    type Buf<_T> = [_T];
    type Output = A::Output;

    fn read(&mut self, rhs: &mut Self::Buf<T>) -> Self::Output {
        rhs[self.symbol].clone()
    }
}

impl<'a, Q, A> Read<&'a [A]> for Head<Q, usize>
where
    Q: RawState,
{
    type Output = &'a A;
    type Error = crate::Error;

    fn read(self, rhs: &'a [A]) -> Result<Self::Output, Self::Error> {
        let pos = self.symbol;
        if pos >= rhs.len() {
            return Err(crate::Error::index_out_of_bounds(pos, rhs.len()));
        }
        Ok(&rhs[pos])
    }
}

impl<'a, Q, A> Read<&'a [A]> for &Head<Q, usize>
where
    Q: RawState,
{
    type Output = &'a A;
    type Error = crate::Error;

    fn read(self, rhs: &'a [A]) -> Result<Self::Output, Self::Error> {
        let pos = self.symbol;
        if pos >= rhs.len() {
            return Err(crate::Error::index_out_of_bounds(pos, rhs.len()));
        }
        Ok(&rhs[pos])
    }
}

impl<'a, Q, A> Read<&'a [A]> for &mut Head<Q, usize>
where
    Q: RawState,
{
    type Output = &'a A;
    type Error = crate::Error;

    fn read(self, rhs: &'a [A]) -> Result<Self::Output, Self::Error> {
        let pos = self.symbol;
        if pos >= rhs.len() {
            return Err(crate::Error::index_out_of_bounds(pos, rhs.len()));
        }
        Ok(&rhs[pos])
    }
}

impl<'a, Q, A> Read<&'a mut [A]> for Head<Q, usize>
where
    Q: RawState,
{
    type Output = &'a A;
    type Error = crate::Error;

    fn read(self, rhs: &'a mut [A]) -> Result<Self::Output, Self::Error> {
        let pos = self.symbol;
        if pos >= rhs.len() {
            return Err(crate::Error::index_out_of_bounds(pos, rhs.len()));
        }
        Ok(&rhs[pos])
    }
}

impl<'a, Q, A> Read<&'a mut [A]> for &Head<Q, usize>
where
    Q: RawState,
{
    type Error = crate::Error;
    type Output = &'a A;

    fn read(self, rhs: &'a mut [A]) -> Result<Self::Output, Self::Error> {
        let pos = self.symbol;
        if pos >= rhs.len() {
            return Err(crate::Error::index_out_of_bounds(pos, rhs.len()));
        }
        Ok(&rhs[pos])
    }
}

impl<'a, Q, A> Read<&'a mut [A]> for &mut Head<Q, usize>
where
    Q: RawState,
{
    type Error = crate::Error;
    type Output = &'a A;

    fn read(self, rhs: &'a mut [A]) -> Result<Self::Output, Self::Error> {
        let pos = self.symbol;
        if pos >= rhs.len() {
            return Err(crate::Error::index_out_of_bounds(pos, rhs.len()));
        }
        Ok(&rhs[pos])
    }
}
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

impl<Q, A, R, B> core::ops::Add<Tail<R, B>> for Head<Q, A>
where
    Q: RawState,
    R: RawState,
{
    type Output = Rule<Q, A, R, B>;

    fn add(self, rhs: Tail<R, B>) -> Self::Output {
        Rule::new(self, rhs)
    }
}

impl<Q, A> core::borrow::Borrow<State<Q>> for Head<Q, A>
where
    Q: RawState,
{
    fn borrow(&self) -> &State<Q> {
        &self.state
    }
}

impl<Q, A> core::borrow::BorrowMut<State<Q>> for Head<Q, A>
where
    Q: RawState,
{
    fn borrow_mut(&mut self) -> &mut State<Q> {
        &mut self.state
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
