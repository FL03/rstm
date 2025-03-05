/*
    Appellation: impl_state <module>
    Contrib: @FL03
*/
use crate::state::{Halt, RawState, State};

impl<Q> State<Q> {
    pub fn from_value(state: Q) -> Self {
        Self(state)
    }
    /// Returns a new instance of state with a raw pointer to the inner value.
    pub fn as_ptr(&self) -> *const Q {
        core::ptr::addr_of!(self.0)
    }
    /// Returns a new instance of state with a mutable raw pointer to the inner value.
    pub fn as_mut_ptr(&mut self) -> *mut Q {
        core::ptr::addr_of_mut!(self.0)
    }
    /// Casts the state to a new type, returning a new instance of [State].
    ///
    /// # Saftey
    ///
    /// This method is unsafe because it is up to the caller to ensure that the cast is valid.
    pub unsafe fn cast<R>(self) -> State<R> {
        unsafe { State(core::ptr::read(&self.0 as *const Q as *const R)) }
    }
    /// Returns an immutable reference to the inner value of the state.
    pub const fn get(&self) -> &Q {
        &self.0
    }
    /// Returns a mutable reference to the inner value of the state.
    pub fn get_mut(&mut self) -> &mut Q {
        self.as_mut()
    }
    #[inline]
    /// Consumes and returns the inner value of the state.
    pub fn into_inner(self) -> Q {
        self.0
    }
    /// [State::map] applies the given function onto the inner value of the state, returning a
    /// new state with the result.
    pub fn map<R, F>(self, f: F) -> State<R>
    where
        F: FnOnce(Q) -> R,
    {
        State(f(self.into_inner()))
    }
    ///
    pub fn map_mut<R, F>(&mut self, mut f: F) -> State<R>
    where
        F: FnMut(&mut Q) -> R,
    {
        State(f(self.get_mut()))
    }
    /// Replaces the state with a new value, returning the old value.
    pub fn replace(&mut self, state: Q) -> Q {
        core::mem::replace(&mut self.0, state)
    }
    /// Clears the state, setting it to its default value.
    pub fn reset(&mut self)
    where
        Q: Default,
    {
        self.set(Default::default());
    }
    /// Sets the state to a new value.
    pub fn set(&mut self, state: Q) {
        self.0 = state;
    }
    /// Swaps the inner value of the state with that of the given state.
    pub fn swap<S>(&mut self, other: &mut S)
    where
        S: RawState<Q = Q>,
    {
        core::mem::swap(&mut self.0, other.get_mut());
    }
    /// Takes the inner value of the state, replacing it with the default value and returning
    /// the previous value.
    pub fn take(&mut self) -> Q
    where
        Q: Default,
    {
        core::mem::take(&mut self.0)
    }
    /// Returns a halted state with an immutable reference to the state.
    pub fn as_halt(&self) -> State<Halt<&Q>> {
        State(Halt(self))
    }
    /// Consumes the state and returns a halted state.
    pub fn into_halt(self) -> State<Halt<Q>> {
        State(Halt(self.into_inner()))
    }
    /// Wraps the inner value with a [Halt] state, returning a new instance of [State].
    pub fn halt(self) -> State<Halt<Q>> {
        State(Halt(self.0))
    }
    /// Returns `true` if the state is a [Halt] state.
    pub fn is_halt(&self) -> bool
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Self>() == core::any::TypeId::of::<State<Halt<Q>>>()
    }
    /// Returns a new state with a boxed inner value.
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
        State(Box::new(self.into_inner()))
    }
    #[cfg(feature = "std")]
    /// Wraps the inner value of the state with an [`Arc`] and returns a new instance of [State]
    pub fn shared(self) -> State<std::sync::Arc<Q>> {
        self.map(std::sync::Arc::new)
    }
    #[cfg(feature = "std")]
    /// Returns a shared reference to the state.
    pub fn to_shared(&self) -> State<std::sync::Arc<Q>>
    where
        Q: Clone,
    {
        self.clone().shared()
    }
    /// Returns a state with an owned inner value.
    pub fn to_ref(&self) -> State<&Q> {
        State(self.get())
    }
    /// Returns a state with a mutable reference to the inner value.
    pub fn to_mut(&mut self) -> State<&mut Q> {
        State(self.get_mut())
    }
    /// Returns the `name` of the generic inner type, `Q`.
    pub fn get_inner_type_name(&self) -> &'static str {
        core::any::type_name::<Q>()
    }
    /// Returns the `type id` of the generic inner type, `Q`.
    pub fn get_inner_type_id(&self) -> core::any::TypeId
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Q>()
    }
}

/*
 ************* References *************
*/
impl<Q> core::convert::AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        &self.0
    }
}

impl<Q> core::convert::AsMut<Q> for State<Q> {
    fn as_mut(&mut self) -> &mut Q {
        &mut self.0
    }
}

impl<Q> core::borrow::Borrow<Q> for State<Q> {
    fn borrow(&self) -> &Q {
        &self.0
    }
}

impl<Q> core::borrow::BorrowMut<Q> for State<Q> {
    fn borrow_mut(&mut self) -> &mut Q {
        &mut self.0
    }
}

impl<Q> core::ops::Deref for State<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Q> core::ops::DerefMut for State<Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/*
 ************* Comparisons *************
*/
impl<Q> core::cmp::PartialEq<Q> for State<Q>
where
    Q: core::cmp::PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.get().eq(other)
    }
}

impl<Q> core::cmp::PartialOrd<Q> for State<Q>
where
    Q: core::cmp::PartialOrd<Q>,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

/*
 ************* Conversions *************
*/
impl<Q> From<Q> for State<Q> {
    fn from(state: Q) -> Self {
        State(state)
    }
}

/*
 ************* Markers *************
*/
unsafe impl<Q> core::marker::Send for State<Q> where Q: core::marker::Send {}

unsafe impl<Q> core::marker::Sync for State<Q> where Q: core::marker::Sync {}

impl_fmt!(State(
    Binary, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex
));
