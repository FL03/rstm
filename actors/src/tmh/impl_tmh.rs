/*
    Appellation: impl_tmh <module>
    Created At: 2025.08.31:00:34:06
    Contrib: @FL03
*/
use super::TMH;
use crate::{Actor, Handle, RawActor};
use rstm_core::{Direction, Head, Symbolic, Tail};
use rstm_state::{RawState, State};

impl<Q, A> RawActor for TMH<Q, A>
where
    Q: RawState,
{
    type Store = Vec<A>;

    seal!();
}

impl<Q, A> Actor for TMH<Q, A> where Q: RawState {}

impl<Q, S> Handle<(Direction, State<Q>, S)> for TMH<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(&mut self, (direction, state, symbol): (Direction, State<Q>, S)) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> Handle<(Direction, Head<Q, S>)> for TMH<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(
        &mut self,
        (direction, Head { state, symbol }): (Direction, Head<Q, S>),
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> Handle<Tail<Q, S>> for TMH<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(
        &mut self,
        Tail {
            direction,
            next_state: state,
            write_symbol: symbol,
        }: Tail<Q, S>,
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> core::fmt::Debug for TMH<Q, S>
where
    Q: RawState,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for (i, c) in self.tape.iter().enumerate() {
            if i == self.position() {
                write!(f, "[{c:?}]")?;
            } else {
                write!(f, "{c:?}")?;
            }
        }
        Ok(())
    }
}

impl<Q, S> core::fmt::Display for TMH<Q, S>
where
    Q: RawState,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for (i, c) in self.tape().iter().enumerate() {
            if i == self.position() {
                write!(f, "[{c}]")?;
            } else {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}
