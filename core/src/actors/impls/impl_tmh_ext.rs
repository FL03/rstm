/*
    Appellation: impl_tmh_ext <module>
    Created At: 2025.08.31:14:54:33
    Contrib: @FL03
*/
#![cfg(feature = "alloc")]

use crate::actors::{Driver, RawDriver, TMH};
use crate::{DEFAULT_DISPLAY_RADIUS, Direction, Head, Tail, get_range_around};
use alloc::vec::Vec;
use rstm_state::{RawState, State};
use rstm_traits::{ExecuteMut, Handle, ReadInto, WriteInto};

impl<Q, A> core::fmt::Debug for TMH<Q, A>
where
    Q: RawState,
    A: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.pretty_print(DEFAULT_DISPLAY_RADIUS).as_str())
    }
}

impl<Q, A> core::fmt::Display for TMH<Q, A>
where
    Q: RawState,
    A: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let pos = self.current_position();
        let (a, b) = get_range_around(pos, self.len(), DEFAULT_DISPLAY_RADIUS);
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

impl<Q, A> ReadInto<A> for TMH<Q, A>
where
    Q: RawState,
    A: Clone,
{
    type Buf<_T> = [_T];
    type Output = Option<usize>;

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    fn read(&mut self, buf: &mut Self::Buf<A>) -> Self::Output {
        #[cfg(feature = "tracing")]
        tracing::trace!("reading the tape...");
        let pos = self.current_position();
        if pos >= self.len() {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "[Index Error] the current position ({pos}) of the head is out of bounds...",
                pos = self.current_position()
            );
            return None;
        }
        let len = buf.len().min(self.len() - pos);
        buf[..len].clone_from_slice(&self.tape()[pos..pos + len]);
        Some(len)
    }
}

impl<Q, A> WriteInto<A> for TMH<Q, A>
where
    Q: RawState,
    A: Clone,
{
    type Buf<'a, _T> = [_T];
    type Output = Option<usize>;

    fn write(&mut self, buf: &mut Self::Buf<'_, A>) -> Self::Output {
        let pos = self.current_position();
        if pos > self.len() {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "[Index Error] the current position ({pos}) of the head is out of bounds...",
                pos = self.current_position()
            );
            return None;
        }
        let len = buf.len();
        if pos + len <= self.len() {
            #[cfg(feature = "tracing")]
            tracing::trace!("Updating the tape at {pos}");
            self.tape_mut()[pos..pos + len].clone_from_slice(buf);
        } else {
            #[cfg(feature = "tracing")]
            tracing::trace!("Extending the tape...");
            // append to the tape
            self.tape_mut().extend_from_slice(buf);
        }
        Some(len)
    }
}

impl<Q, A> RawDriver<Q, A> for TMH<Q, A>
where
    Q: RawState,
{
    type Tape<_T> = Vec<_T>;

    seal! {}

    fn current_state(&self) -> State<&Q> {
        self.head().state().view()
    }

    fn tape(&self) -> &Self::Tape<A> {
        &self.tape
    }
}

impl<Q, A> Driver<Q, A> for TMH<Q, A>
where
    Q: RawState,
{
    fn store_mut(&mut self) -> &mut Self::Tape<A> {
        &mut self.tape
    }
}

impl<Q, A> ExecuteMut<(Direction, State<Q>, A)> for TMH<Q, A>
where
    Q: RawState,
{
    type Output = crate::Result<Head<Q, usize>>;

    fn execute(&mut self, (direction, state, symbol): (Direction, State<Q>, A)) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, A> ExecuteMut<(Direction, Head<Q, A>)> for TMH<Q, A>
where
    Q: RawState,
{
    type Output = crate::Result<Head<Q, usize>>;

    fn execute(
        &mut self,
        (direction, Head { state, symbol }): (Direction, Head<Q, A>),
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, A> ExecuteMut<Tail<Q, A>> for TMH<Q, A>
where
    Q: RawState,
{
    type Output = crate::Result<Head<Q, usize>>;

    fn execute(
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
