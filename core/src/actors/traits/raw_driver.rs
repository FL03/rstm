/*
    Appellation: raw_driver <module>
    Created At: 2025.08.31:00:17:44
    Contrib: @FL03
*/
use crate::Tail;
use rspace_traits::RawSpace;
use rstm_state::State;
use rstm_traits::Handle;

/// The [`RawDriver`] is the basis for all compatible actors within the system. Each
/// implementation is required to define the _type_ of internal store it will use to
/// manage its data. This abstraction allows for flexibility in the choice of data structures,
/// enabling the actor to adapt to various use cases and performance requirements.
pub trait RawDriver<Q, A> {
    type Tape<_T>: RawSpace<Elem = _T>;

    /// returns a reference to the current state of the driver
    fn current_state(&self) -> State<&Q>;
    /// returns a reference to the driver's internal store
    fn store(&self) -> &Self::Tape<A>;

    private! {}
}

/// Here, an [`Driver`] defines an entity capable of performing actions within an environment,
/// typically in response to a set of rules or stimuli. The interface works to generalize the
/// core components of the actual machines, isoating the essential behaviors and interactions
/// that define an actor's role within a system.
///
/// If Turing machines were divided into two focuses we could define them as the rulespace and
/// the actor. Understanding this distinction provides crucial insights into the design and
/// functionality of the machines. Most importantly, it clarifies the role of the actor as the
/// an entity without any inherent context or logic, rather, it is simply an _actionable_
/// system that may be orchestrated according to a set of rules. Any alterations to the
/// rulespace have an immediate effect on the overall behavior of the actor, as it is the
/// rules themselves that dictate the _response_ of the system depending on the current state
/// and symbol being read.
pub trait Driver<Q, A>: RawDriver<Q, A> + Handle<Tail<Q, A>> {
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
