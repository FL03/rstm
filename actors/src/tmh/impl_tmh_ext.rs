/*
    Appellation: impl_tmh_ext <module>
    Created At: 2025.08.31:14:54:33
    Contrib: @FL03
*/
use super::TMH;
use crate::{Actor, Handle, RawActor};
use rstm_core::{Direction, Head, Tail, get_range_around};
use rstm_state::{RawState, State};

const DISPLAY_RADIUS: usize = 10;

impl<Q, A> core::fmt::Debug for TMH<Q, A>
where
    Q: RawState,
    A: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let pos = self.current_position();
        let (a, b) = get_range_around(pos, self.len(), DISPLAY_RADIUS);
        // print out the tape with the head position highlighted
        for (idx, c) in (a..=b).zip(self.tape[a..=b].iter()) {
            let cell = if pos == idx || (idx == b && pos == (idx + 1)) {
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
        let pos = self.current_position();
        let (a, b) = get_range_around(pos, self.len(), DISPLAY_RADIUS);
        // print out the tape with the head position highlighted
        for (idx, c) in (a..=b).zip(self.tape[a..=b].iter()) {
            let cell = if pos == idx || (idx == b && pos == (idx + 1)) {
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
    type Store<_T> = Vec<_T>;

    seal!();
}

impl<Q, A> Actor<Q, A> for TMH<Q, A>
where
    Q: RawState,
{
    fn store(&self) -> &Self::Store<A> {
        &self.tape
    }

    fn store_mut(&mut self) -> &mut Self::Store<A> {
        &mut self.tape
    }
}

impl<Q, A> Handle<(Direction, State<Q>, A)> for TMH<Q, A>
where
    Q: RawState,
{
    type Output = crate::Result<Head<Q, usize>>;

    fn handle(&mut self, (direction, state, symbol): (Direction, State<Q>, A)) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, A> Handle<(Direction, Head<Q, A>)> for TMH<Q, A>
where
    Q: RawState,
{
    type Output = crate::Result<Head<Q, usize>>;

    fn handle(
        &mut self,
        (direction, Head { state, symbol }): (Direction, Head<Q, A>),
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, A> Handle<Tail<Q, A>> for TMH<Q, A>
where
    Q: RawState,
{
    type Output = crate::Result<Head<Q, usize>>;

    fn handle(
        &mut self,
        Tail {
            direction,
            next_state: state,
            write_symbol: symbol,
        }: Tail<Q, A>,
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}
