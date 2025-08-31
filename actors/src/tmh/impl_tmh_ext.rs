/*
    Appellation: impl_tmh_ext <module>
    Created At: 2025.08.31:14:54:33
    Contrib: @FL03
*/
use super::TMH;
use crate::{Actor, Handle, RawActor};
use rstm_core::{Direction, Head, Symbolic, Tail, get_range_around};
use rstm_state::{RawState, State};

const DISPLAY_RADIUS: usize = 10;

impl<Q, A> core::fmt::Debug for TMH<Q, A>
where
    Q: RawState,
    A: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let (a, b) = get_range_around(self.position(), self.len(), DISPLAY_RADIUS);
        // print out the tape with the head position highlighted
        for (i, c) in self.tape[a..=b].iter().enumerate() {
            let cell = if i == self.position() {
                format!("[{c:?}]")
            } else {
                format!("{c:?}")
            };
            f.write_str(&cell)?;
        }
        Ok(())
    }
}

impl<Q, A> core::fmt::Display for TMH<Q, A>
where
    Q: RawState,
    A: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let (a, b) = get_range_around(self.position(), self.len(), DISPLAY_RADIUS);
        // print out the tape with the head position highlighted
        for (i, c) in self.tape[a..=b].iter().enumerate() {
            let cell = if i == self.position() {
                format!("[{c}]")
            } else {
                format!("{c}")
            };
            f.write_str(&cell)?;
        }
        Ok(())
    }
}

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
    type Output = crate::Result<Head<Q, usize>>;

    fn handle(&mut self, (direction, state, symbol): (Direction, State<Q>, S)) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> Handle<(Direction, Head<Q, S>)> for TMH<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Output = crate::Result<Head<Q, usize>>;

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
    type Output = crate::Result<Head<Q, usize>>;

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
