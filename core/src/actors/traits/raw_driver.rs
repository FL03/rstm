/*
    Appellation: raw_driver <module>
    Created At: 2025.08.31:00:17:44
    Contrib: @FL03
*/
use crate::rules::{Head, Tail};
use rspace_traits::RawSpace;
use rstm_state::{RawState, State};
use rstm_traits::{Read, TryExecute};

/// The [`Driver`] is the basis for all compatible actors within the system. Each
/// implementation is required to define the _type_ of internal store it will use to
/// manage its data. This abstraction allows for flexibility in the choice of data structures,
/// enabling the actor to adapt to various use cases and performance requirements.
pub trait Driver<Q, A>
where
    for<'a> Self: Read<&'a mut [A], Output = &'a A>,
{
    private! {}
    /// returns the current position of the driver.
    fn current_position(&self) -> usize;
    /// returns a view of the current state of the driver.
    fn current_state(&self) -> State<&Q>;
}

/// An [`Actor`] is a particular kind of driver capable of maintaining its own internal store.
pub trait Actor<Q, A>: Driver<Q, A> + TryExecute<Tail<Q, A>>
where
    for<'a> Self: Read<&'a mut [A], Output = &'a A>,
{
    type Tape<_T>: RawSpace<Elem = _T>;
    /// returns a reference to the driver's internal store
    fn store(&self) -> &Self::Tape<A>;
    /// returns a mutable reference to the internal store
    fn store_mut(&mut self) -> &mut Self::Tape<A>;
    /// [`replace`](core::mem::replace) the current store with another before returning the previous value
    fn replace_store(&mut self, store: Self::Tape<A>) -> Self::Tape<A> {
        core::mem::replace(self.store_mut(), store)
    }
    /// set the store to the given value
    fn set_store(&mut self, store: Self::Tape<A>) {
        *self.store_mut() = store;
    }
    /// [`swap`](core::mem::swap) the current store with another
    fn swap_store(&mut self, store: &mut Self::Tape<A>) {
        core::mem::swap(self.store_mut(), store);
    }
    /// [`take`](core::mem::take) the current store, leaving the default value in its place
    fn take_store(&mut self) -> Self::Tape<A>
    where
        Self::Tape<A>: Default,
    {
        core::mem::take(self.store_mut())
    }
}

impl<Q, A> Driver<Q, A> for Head<Q, usize>
where
    Q: RawState,
    for<'a> Self: Read<&'a mut [A], Output = &'a A>,
{
    seal! {}

    fn current_position(&self) -> usize {
        self.symbol
    }

    fn current_state(&self) -> State<&Q> {
        self.state.view()
    }
}
