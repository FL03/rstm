/*
    Appellation: impl_tmh_ext <module>
    Created At: 2025.08.31:14:54:33
    Contrib: @FL03
*/
#![cfg(feature = "alloc")]

use crate::actors::drivers::TMH;
use crate::actors::{Driver, RawDriver};
use crate::{Direction, Head, Tail};
use alloc::vec::Vec;
use rstm_state::{RawState, State};
use rstm_traits::{Handle, Read, TryExecuteMut, Write};

impl<Q, A> core::fmt::Debug for TMH<Q, A>
where
    Q: RawState,
    A: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.pretty_print().as_str())
    }
}

impl<Q, A> core::fmt::Display for TMH<Q, A>
where
    Q: RawState,
    A: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.print().as_str())
    }
}

impl<Q, A> Read<A> for TMH<Q, A>
where
    Q: RawState,
    A: Clone,
{
    type Error = crate::Error;
    type Output = usize;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "read", target = "tmh")
    )]
    fn read(&mut self, buf: &mut [A]) -> Result<Self::Output, Self::Error> {
        #[cfg(feature = "tracing")]
        tracing::trace! { "reading the tape..." }
        let pos = self.current_position();
        if pos >= self.len() {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "[Index Error] the current position ({pos}) of the head is out of bounds...",
                pos = self.current_position()
            );
            return Err(crate::Error::IndexOutOfBounds {
                index: pos,
                len: self.len(),
            });
        }
        let len = buf.len().min(self.len() - pos);
        buf[..len].clone_from_slice(&self.tape()[pos..pos + len]);
        Ok(len)
    }
}

impl<Q, A> Write<A> for TMH<Q, A>
where
    Q: RawState,
    A: Clone,
{
    type Error = crate::Error;
    type Output = usize;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "write", target = "tmh")
    )]
    fn write(&mut self, buf: &mut [A]) -> Result<Self::Output, Self::Error> {
        let pos = self.current_position();
        if pos > self.len() {
            #[cfg(feature = "tracing")]
            tracing::error! {
                "[Index Error] the current position ({}) of the head is out of bounds for tape of length {}",
                pos, self.len(),
            };
            return Err(crate::Error::IndexOutOfBounds {
                index: pos,
                len: self.len(),
            });
        }
        let len = buf.len();
        if pos + len <= self.len() {
            #[cfg(feature = "tracing")]
            tracing::trace! { "Updating the tape at {pos}" };
            self.tape_mut()[pos..pos + len].clone_from_slice(buf);
        } else {
            #[cfg(feature = "tracing")]
            tracing::trace!("Extending the tape...");
            // append to the tape
            self.tape_mut().extend_from_slice(buf);
        }
        Ok(len)
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
        &self.input
    }
}

impl<Q, A> Driver<Q, A> for TMH<Q, A>
where
    Q: RawState,
{
    fn store_mut(&mut self) -> &mut Self::Tape<A> {
        &mut self.input
    }
}

impl<X, Y, E, Q, A> Handle<X> for TMH<Q, A>
where
    Q: RawState,
    Self: TryExecuteMut<X, Error = E, Output = Y>,
{
    type Output = Result<Y, E>;

    fn handle(&mut self, rhs: X) -> Self::Output {
        self.try_execute(rhs)
    }
}


impl<Q, A> TryExecuteMut<(Direction, State<Q>, A)> for TMH<Q, A>
where
    Q: RawState,
{
    type Error = crate::Error;
    type Output = Head<Q, usize>;

    fn try_execute(
        &mut self,
        (direction, state, symbol): (Direction, State<Q>, A),
    ) -> Result<Self::Output, Self::Error> {
        self.step(direction, state, symbol)
    }
}

impl<Q, A> TryExecuteMut<(Direction, Head<Q, A>)> for TMH<Q, A>
where
    Q: RawState,
{
    type Error = crate::Error;
    type Output = Head<Q, usize>;

    fn try_execute(
        &mut self,
        (direction, Head { state, symbol }): (Direction, Head<Q, A>),
    ) -> Result<Self::Output, Self::Error> {
        self.step(direction, state, symbol)
    }
}

impl<Q, A> TryExecuteMut<Tail<Q, A>> for TMH<Q, A>
where
    Q: RawState,
{
    type Error = crate::Error;
    type Output = Head<Q, usize>;

    fn try_execute(
        &mut self,
        Tail {
            direction,
            next_state: state,
            write_symbol: symbol,
        }: Tail<Q, A>,
    ) -> Result<Self::Output, Self::Error> {
        self.step(direction, state, symbol)
    }
}