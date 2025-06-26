/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

mod impl_state;
#[allow(deprecated)]
mod impl_state_deprecated;
mod impl_state_ops;
mod impl_state_repr;

use super::{Halt, Halter, RawState};

/// [State] is a generalized state implementation, representing the state of a system or
/// object.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct State<Q: ?Sized = bool>(pub Q);

impl<Q> State<Q>
where
    Q: RawState,
{
    /// a constructor method for the [`State`] type.
    pub const fn new(state: Q) -> Self {
        Self(state)
    }
    /// initializes a new instance of state using the given initializer function.
    pub fn create<F>(f: F) -> Self
    where
        F: FnOnce() -> Q,
    {
        State(f())
    }
    /// returns a new state with a value of one.
    pub fn one() -> Self
    where
        Q: num_traits::One,
    {
        State::create(Q::one)
    }
    /// returns a new state with a value of zero.
    pub fn zero() -> Self
    where
        Q: num_traits::Zero,
    {
        State::create(Q::zero)
    }
    /// returns a new instance of state with a raw pointer to the inner value.
    pub const fn as_ptr(&self) -> *const Q {
        core::ptr::addr_of!(self.0)
    }
    /// returns a new instance of state with a mutable raw pointer to the inner value.
    pub const fn as_mut_ptr(&mut self) -> *mut Q {
        core::ptr::addr_of_mut!(self.0)
    }
    #[allow(clippy::missing_safety_doc)]
    /// Casts the state to a new type, returning a new instance of [State].
    ///
    /// # Saftey
    ///
    /// This method is unsafe because it is up to the caller to ensure that the cast is valid.
    pub unsafe fn cast<R>(self) -> State<R> {
        unsafe { State(core::ptr::read(&self.0 as *const Q as *const R)) }
    }
    /// returns an immutable reference to the inner value of the state.
    pub const fn get(&self) -> &Q {
        &self.0
    }
    /// returns a mutable reference to the inner value of the state.
    pub const fn get_mut(&mut self) -> &mut Q {
        &mut self.0
    }
    /// consumes and returns the inner value of the state.
    #[inline]
    pub fn value(self) -> Q {
        self.0
    }
    /// [State::map] applies the given function onto the inner value of the state, returning a
    /// new state with the result.
    pub fn map<R, F>(self, f: F) -> State<R>
    where
        F: FnOnce(Q) -> R,
    {
        State(f(self.value()))
    }
    /// [`replace`](core::mem::replace) the inner value of the state with the given state,
    pub const fn replace(&mut self, state: Q) -> Q {
        core::mem::replace(self.get_mut(), state)
    }
    /// Clears the state, setting it to its default value.
    #[inline]
    pub fn reset(&mut self) -> &mut Self
    where
        Q: Default,
    {
        self.set(Default::default());
        self
    }
    /// Sets the state to a new value.
    pub fn set(&mut self, state: Q) -> &mut Self {
        self.0 = state;
        self
    }
    /// [`swap`](core::mem::swap) the inner value of the state with that of the given state.
    pub const fn swap(&mut self, other: &mut State<Q>) {
        core::mem::swap(self.get_mut(), other.get_mut());
    }
    /// [`take`](core::mem::take) the inner value of the state, leaving the logical default in
    /// its place
    pub fn take(&mut self) -> Q
    where
        Q: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// consumes the wrapper to create another, haltable state that is initialized with the
    /// current state
    pub fn into_halter(self) -> State<Halter<Q>> {
        State::new(Halter::state(self))
    }
    /// converts the current reference into a haltable state initialized with the current state
    pub fn as_halt(&self) -> State<Halt<&Q>> {
        self.view().map(Halt)
    }
    /// consumes the current state, returning a new one with a [`Halt`] wrapper around the
    /// inner value.
    pub fn halt(self) -> State<Halt<Q>> {
        self.map(Halt)
    }
    /// returns a new state with a boxed inner value.
    pub fn boxed(self) -> State<Box<Q>> {
        self.map(Box::new)
    }
    /// Converts the inner type into a boxed "any" state, returning a new instance of state
    pub fn as_any(&self) -> State<Box<dyn std::any::Any>>
    where
        Q: Clone + 'static,
    {
        State(Box::new(self.get().clone()))
    }
    /// Converts the inner type into a boxed "any" state, returning a new instance of state
    pub fn into_any(self) -> State<Box<dyn std::any::Any>>
    where
        Q: 'static,
    {
        State(Box::new(self.value()))
    }

    #[cfg(feature = "std")]
    /// Wraps the inner value of the state with an [`Arc`] and returns a new instance of [State]
    pub fn shared(self) -> State<std::sync::Arc<Q>> {
        self.map(std::sync::Arc::new)
    }
    #[cfg(feature = "std")]
    /// returns a shared reference to the state.
    pub fn to_shared(&self) -> State<std::sync::Arc<Q>>
    where
        Q: Clone,
    {
        self.clone().shared()
    }
    /// returns a state with an owned inner value.
    pub const fn view(&self) -> State<&Q> {
        State(self.get())
    }
    /// returns a state with a mutable reference to the inner value.
    pub const fn view_mut(&mut self) -> State<&mut Q> {
        State(self.get_mut())
    }
    /// returns the `name` of the generic inner type, `Q`.
    pub fn get_inner_type_name(&self) -> &'static str {
        core::any::type_name::<Q>()
    }
    /// returns the `type id` of the generic inner type, `Q`.
    pub fn get_inner_type_id(&self) -> core::any::TypeId
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Q>()
    }
}
