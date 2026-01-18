/*
    Appellation: impl_tmh_ext <module>
    Created At: 2025.08.31:14:54:33
    Contrib: @FL03
*/
#![cfg(feature = "alloc")]

use crate::actors::drivers::TMH;
use crate::actors::{Actor, RawDriver};
use crate::{Direction, Head, Tail};
use alloc::vec::Vec;
use rstm_state::{RawState, State};
use rstm_traits::{Handle, Read, TryExecuteMut};

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

impl<E, X, Y, Q, A> Read<X> for TMH<Q, A>
where
    E: core::error::Error,
    Q: RawState,
    Head<Q, usize>: Read<X, Error = E, Output = Y>,
{
    type Output = Y;
    type Error = E;

    fn read(self, rhs: X) -> Result<Self::Output, Self::Error> {
        self.head.read(rhs)
    }
}

impl<E, X, Y, Q, A> Read<X> for &TMH<Q, A>
where
    E: core::error::Error,
    Q: RawState,
    for<'b> &'b Head<Q, usize>: Read<X, Error = E, Output = Y>,
{
    type Output = Y;
    type Error = E;

    fn read(self, rhs: X) -> Result<Self::Output, Self::Error> {
        self.head().read(rhs)
    }
}

impl<E, X, Y, Q, A> Read<X> for &mut TMH<Q, A>
where
    E: core::error::Error,
    Q: RawState,
    for<'b> &'b mut Head<Q, usize>: Read<X, Error = E, Output = Y>,
{
    type Output = Y;
    type Error = E;

    fn read(self, rhs: X) -> Result<Self::Output, Self::Error> {
        self.head_mut().read(rhs)
    }
}

impl<Q, A> RawDriver<Q, A> for TMH<Q, A>
where
    Q: RawState,
{
    seal! {}

    fn current_position(&self) -> usize {
        self.head().symbol
    }

    fn current_state(&self) -> State<&Q> {
        self.head().state().view()
    }
}

impl<Q, A> Actor<Q, A> for TMH<Q, A>
where
    Q: RawState,
{
    type Tape<_T> = Vec<_T>;

    fn store(&self) -> &Self::Tape<A> {
        &self.input
    }

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
