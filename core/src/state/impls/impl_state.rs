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
    /// returns a new state with a value of one.
    pub fn one() -> Self
    where
        Q: num_traits::One,
    {
        State(Q::one())
    }
    /// returns a new state with a value of zero.
    pub fn zero() -> Self
    where
        Q: num_traits::Zero,
    {
        State(Q::zero())
    }
    /// returns a new instance of state with a raw pointer to the inner value.
    pub const fn as_ptr(&self) -> *const Q {
        core::ptr::addr_of!(self.0)
    }
    /// returns a new instance of state with a mutable raw pointer to the inner value.
    pub const fn as_mut_ptr(&mut self) -> *mut Q {
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
    /// returns a halted state with an immutable reference to the state.
    pub fn as_halt(&self) -> State<Halt<&Q>> {
        State(Halt(self))
    }
    /// Consumes the state and returns a halted state.
    pub fn into_halt(self) -> State<Halt<Q>> {
        State(Halt(self.value()))
    }
    /// Wraps the inner value with a [Halt] state, returning a new instance of [State].
    pub fn halt(self) -> State<Halt<Q>> {
        State(Halt(self.0))
    }
    /// returns `true` if the state is a [Halt] state.
    pub fn is_halt(&self) -> bool
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Self>() == core::any::TypeId::of::<State<Halt<Q>>>()
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

impl<Q> State<Q> {
    #[deprecated(
        since = "0.0.7",
        note = "use `value` instead, as it is more idiomatic and clearer."
    )]
    pub fn into_inner(self) -> Q {
        self.0
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_ref(&self) -> State<&Q> {
        self.view()
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view_mut` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_mut(&mut self) -> State<&mut Q> {
        self.view_mut()
    }
}

/*
 ************* References *************
*/
impl<Q> core::convert::AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        self.get()
    }
}

impl<Q> core::convert::AsMut<Q> for State<Q> {
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

/*
 ************* Comparisons *************
*/
impl<Q> core::cmp::PartialEq<Q> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.get().eq(other)
    }
}

impl<Q> core::cmp::PartialOrd<Q> for State<Q>
where
    Q: PartialOrd,
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

macro_rules! impl_fmt {
    ($wrap:ident<$T:ident>($($trait:ident),* $(,)?)) => {
        $(
            impl_fmt!(@impl $wrap<$T>($trait));
        )*
    };
    (@impl $wrap:ident<$T:ident>($trait:ident)) => {
        impl<$T> ::core::fmt::$trait for State<$T>
        where
            $T: ::core::fmt::$trait,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                ::core::fmt::$trait::fmt(self.get(), f)
            }
        }
    };
}

impl_fmt! {
    State<Q>(
        Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex
    )
}
