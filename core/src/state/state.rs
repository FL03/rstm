/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Halt, RawState};
use crate::Error;
use std::sync::Arc;
///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct State<Q: ?Sized = usize>(pub Q);

impl<Q> State<Q> {
    pub fn new(state: Q) -> Self {
        Self(state)
    }
    /// Casts the state to a new type, returning a new instance of [State].
    ///
    /// ### Saftey
    ///
    /// *
    pub unsafe fn cast<R>(self) -> State<R> {
        State(core::ptr::read(&self.0 as *const Q as *const R))
    }
    #[inline]
    /// Consumes and returns the inner value of the state.
    pub fn get(self) -> Q {
        self.0
    }
    /// Returns an immutable reference to the inner value of the state.
    pub const fn get_ref(&self) -> &Q {
        &self.0
    }
    /// Returns a mutable reference to the inner value of the state.
    pub fn get_mut(&mut self) -> &mut Q {
        self.as_mut()
    }
    /// [State::map] applies the given function onto the inner value of the state, returning a
    /// new state with the result. Essentially, the method describes the mechanism for
    /// transfroming the state.
    pub fn map<R, F>(self, f: F) -> State<R>
    where
        F: FnOnce(Q) -> R,
    {
        State(f(self.get()))
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
        State(Halt(self.get()))
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
        State(Box::new(self.get_ref().clone()))
    }
    /// Converts the inner type into a boxed "any" state, returning a new instance of state
    pub fn into_any(self) -> State<Box<dyn std::any::Any>>
    where
        Q: 'static,
    {
        State(Box::new(self.get()))
    }
    /// Wraps the inner value of the state with an [`Arc`] and returns a new instance of [State]
    pub fn shared(self) -> State<Arc<Q>> {
        self.map(Arc::new)
    }
    /// Returns a shared reference to the state.
    pub fn to_shared(&self) -> State<Arc<Q>>
    where
        Q: Clone,
    {
        State(Arc::new(self.get_ref().clone()))
    }
    /// Returns a state with an owned inner value.
    pub fn to_ref(&self) -> State<&Q> {
        State(self.get_ref())
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
}

impl State<()> {
    /// Creates a new instance of [State] with an empty state.
    pub fn empty() -> Self {
        Self(())
    }
}

impl State<Box<dyn core::any::Any>> {
    /// Downcasts the state to a new type, returning a new instance of [State].
    pub fn downcast<Q>(self) -> Result<State<Box<Q>>, Error>
    where
        Q: core::any::Any,
    {
        self.get()
            .downcast()
            .map(State)
            .map_err(|_| Error::type_error("Failed to downcast state"))
    }

    pub fn downcast_ref<Q>(&self) -> Option<State<&Q>>
    where
        Q: core::any::Any,
    {
        self.get_ref().downcast_ref().map(State)
    }
}
impl<Q> State<Halt<Q>> {
    /// Creates a new instance of [State] from a [Halt] state.
    pub fn halted(Halt(inner): Halt<Q>) -> Self {
        Self(Halt(inner))
    }

    pub fn unhalt(self) -> State<Q> {
        State(self.0.into_inner())
    }
}

impl<'a, Q> State<&'a Q> {
    /// Clones the internal state and returning a new instance of [State]
    pub fn cloned(&self) -> State<Q>
    where
        Q: Clone,
    {
        State(self.0.clone())
    }
    /// Copies the internal state and returning a new instance of [State]
    pub fn copied(&self) -> State<Q>
    where
        Q: Copy,
    {
        State(*self.0)
    }
}

impl<'a, Q> State<&'a mut Q> {
    /// Clones the internal state and returning a new instance of [State]
    pub fn cloned(&self) -> State<Q>
    where
        Q: Clone,
    {
        State(self.0.clone())
    }
    /// Copies the internal state and returning a new instance of [State]
    pub fn copied(&self) -> State<Q>
    where
        Q: Copy,
    {
        State(*self.0)
    }
}

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

macro_rules! impl_fmt {
    ($($trait:ident),*) => {
        $(
            impl<Q> core::fmt::$trait for State<Q>
            where
                Q: core::fmt::$trait,
            {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    core::fmt::$trait::fmt(&self.0, f)
                }
            }
        )*
    };
}

impl_fmt!(Binary, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex);

unsafe impl<Q> core::marker::Send for State<Q> where Q: core::marker::Send {}

unsafe impl<Q> core::marker::Sync for State<Q> where Q: core::marker::Sync {}

impl<Q> core::cmp::PartialEq<Q> for State<Q>
where
    Q: core::cmp::PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.get_ref().eq(other)
    }
}

impl<Q> core::cmp::PartialOrd<Q> for State<Q>
where
    Q: core::cmp::PartialOrd<Q>,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.get_ref().partial_cmp(other)
    }
}

impl<Q> From<Q> for State<Q> {
    fn from(state: Q) -> Self {
        State(state)
    }
}

macro_rules! impl_ops {
    (@alt $trait:ident::$call:ident) => {
        impl<Q, R> ::core::ops::$trait<R> for State<Q>
        where
            Q: ::core::ops::$trait,
            R: $crate::state::RawState<Inner = Q>,
        {
            type Output = State<Q::Output>;

            fn $call(self, rhs: State<Q>) -> Self::Output {
                State(::core::ops::$trait::$call(self.into_inner(), rhs.into_inner()))
            }
        }
    };
    (@impl $trait:ident::$call:ident) => {
        impl<Q> ::core::ops::$trait for State<Q>
        where
            Q: ::core::ops::$trait,
        {
            type Output = State<Q::Output>;

            fn $call(self, rhs: State<Q>) -> Self::Output {
                State(::core::ops::$trait::$call(self.into_inner(), rhs.into_inner()))
            }
        }

        impl<Q> ::core::ops::$trait<Q> for State<Q>
        where
            Q: core::ops::$trait<Q>,
        {
            type Output = State<Q::Output>;

            fn $call(self, rhs: Q) -> Self::Output {
                State(::core::ops::$trait::$call(self.into_inner(), rhs))
            }
        }

        paste::paste! {
            impl<Q> ::core::ops::[<$trait Assign>]<State<Q>> for State<Q>
            where
                Q: ::core::ops::[<$trait Assign>],
            {

                fn [<$call _assign>](&mut self, rhs: State<Q>) {
                    ::core::ops::[<$trait Assign>]::[<$call _assign>](self.get_mut(), rhs.into_inner())
                }
            }
            impl<Q> ::core::ops::[<$trait Assign>]<Q> for State<Q>
            where
                Q: ::core::ops::[<$trait Assign>],
            {

                fn [<$call _assign>](&mut self, rhs: Q) {
                    ::core::ops::[<$trait Assign>]::[<$call _assign>](self.get_mut(), rhs)
                }
            }
        }
    };
    ($($trait:ident::$call:ident),* $(,)?) => {
        $(
            impl_ops!(@impl $trait::$call);
        )*
    };
}

impl_ops! {
    Add::add,
    BitAnd::bitand,
    BitOr::bitor,
    BitXor::bitxor,
    Div::div,
    Mul::mul,
    Rem::rem,
    Shl::shl,
    Shr::shr,
    Sub::sub,
}
