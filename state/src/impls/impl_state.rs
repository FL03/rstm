/*
    Appellation: impl_state <module>
    Contrib: @FL03
*/
use crate::state::{Halt, State};

impl<Q> State<Q> {
    /// a constructor method for the [`State`] type.
    pub const fn new(state: Q) -> Self {
        Self(state)
    }
    /// create a new state by invoking the given function and capturing its output.
    #[inline]
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
    /// Casts the state to a new type, returning a new instance of [State].
    ///
    /// # Safety
    ///
    /// The method is marked unsafe because there are no guarantees that the inner type `Q`
    /// can be safely interpreted as type `R`. The caller must ensure that the cast is valid
    /// to avoid undefined behavior.
    #[inline]
    pub unsafe fn cast<R>(self) -> State<R> {
        unsafe { State(core::ptr::read(&self.0 as *const Q as *const R)) }
    }
    /// [State::map] applies the given function onto the inner value of the state, returning a
    /// new state with the result.
    pub fn map<R, F>(self, f: F) -> State<R>
    where
        F: FnOnce(Q) -> R,
    {
        State(f(self.value()))
    }
    /// reset the state backl to its logical default and return a mutable reference to the
    /// state itself.
    pub fn reset(&mut self) -> &mut Self
    where
        Q: Default,
    {
        self.set(Default::default());
        self
    }
    /// update the current value of the object with the given state.
    #[inline]
    pub fn set(&mut self, state: Q) {
        self.0 = state
    }
    /// [`replace`](core::mem::replace) the inner value of the state with the given state,
    pub const fn replace(&mut self, state: Q) -> Q {
        core::mem::replace(self.get_mut(), state)
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
    /// converts the current reference into a haltable state initialized with the current state
    pub fn as_halt(&self) -> State<Halt<&Q>> {
        State::new(Halt::Halt(self.get()))
    }
    /// consumes the wrapper to create another, haltable state that is initialized with the
    /// current state
    pub fn into_halt(self) -> State<Halt<Q>> {
        State::new(Halt::Step(self.value()))
    }
    /// consumes the current state, returning a new one with a [`Halt`](Halt::Halt)
    /// variant initialized with the current value.
    pub fn halt(self) -> State<Halt<Q>> {
        State::new(Halt::Halt(self.value()))
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
    pub const fn get_inner_type_id(&self) -> core::any::TypeId
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Q>()
    }
}

#[cfg(feature = "alloc")]
/// Additional implementations for the [`State`] type enabled whenever the `alloc` feature is
/// toggled.
impl<Q> State<Q> {
    /// returns a new state with a boxed inner value.
    pub fn boxed(self) -> State<alloc::boxed::Box<Q>> {
        self.map(alloc::boxed::Box::new)
    }
    /// Converts the inner type into a boxed "any" state, returning a new instance of state
    pub fn as_any(&self) -> State<alloc::boxed::Box<dyn core::any::Any>>
    where
        Q: Clone + 'static,
    {
        State(alloc::boxed::Box::new(self.get().clone()))
    }
    /// Converts the inner type into a boxed "any" state, returning a new instance of state
    pub fn into_any(self) -> State<alloc::boxed::Box<dyn core::any::Any>>
    where
        Q: 'static,
    {
        State(alloc::boxed::Box::new(self.value()))
    }
    /// Wraps the inner value of the state with an [`Arc`] and returns a new instance of [State]
    pub fn shared(self) -> State<alloc::sync::Arc<Q>> {
        self.map(alloc::sync::Arc::new)
    }
    /// returns a shared reference to the state.
    pub fn to_shared(&self) -> State<alloc::sync::Arc<Q>>
    where
        Q: Clone,
    {
        self.clone().shared()
    }
}

impl<Q> AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        self.get()
    }
}

impl<Q> AsMut<Q> for State<Q> {
    fn as_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::borrow::Borrow<Q> for State<Q> {
    fn borrow(&self) -> &Q {
        self.get()
    }
}

impl<Q> core::borrow::BorrowMut<Q> for State<Q> {
    fn borrow_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::ops::Deref for State<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<Q> core::ops::DerefMut for State<Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<Q> From<Q> for State<Q> {
    fn from(state: Q) -> Self {
        State(state)
    }
}

impl<Q: PartialEq> PartialEq<Q> for State<Q> {
    fn eq(&self, other: &Q) -> bool {
        self.0.eq(other)
    }
}

impl<'a, Q: PartialEq> PartialEq<&'a Q> for State<Q> {
    fn eq(&self, other: &&'a Q) -> bool {
        self.0.eq(*other)
    }
}

impl<'a, Q: PartialEq> PartialEq<&'a mut Q> for State<Q> {
    fn eq(&self, other: &&'a mut Q) -> bool {
        self.0.eq(*other)
    }
}

impl<Q: PartialOrd> PartialOrd<Q> for State<Q> {
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<'a, Q: PartialOrd> PartialOrd<&'a Q> for State<Q> {
    fn partial_cmp(&self, other: &&'a Q) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl<'a, Q: PartialOrd> PartialOrd<&'a mut Q> for State<Q> {
    fn partial_cmp(&self, other: &&'a mut Q) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

unsafe impl<Q: Send> Send for State<Q> {}

unsafe impl<Q: Sync> Sync for State<Q> {}
