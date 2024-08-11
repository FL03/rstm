/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::Halt;
use std::sync::Arc;
///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct State<Q = String>(pub Q);

impl<Q> State<Q> {
    pub fn new(state: Q) -> Self {
        Self(state)
    }
    /// Returns an immutable reference to the state.
    pub fn as_ref(&self) -> &Q {
        &self.0
    }
    /// Returns a mutable reference to the state.
    pub fn as_mut(&mut self) -> &mut Q {
        &mut self.0
    }
    /// Consumes the state and returns a boxed state.
    pub fn boxed(self) -> State<Box<Q>> {
        State(Box::new(self.0))
    }
    /// Consumes and returns the inner value of the [state](State).
    pub fn into_inner(self) -> Q {
        self.0
    }
    /// Consumes the state and returns an owned state.
    pub fn into_owned(self) -> State<Q>
    where
        Q: Clone,
    {
        State(self.0.clone())
    }
    /// Transforms the state into a shared reference.
    pub fn into_shared(self) -> State<Arc<Q>> {
        State(Arc::new(self.0))
    }
    /// [State::map] applies a [`Fn`] closure to the state, returing a new state in the process.
    /// Essentially, the method sufficiently describes the transformation of the state.
    pub fn map<R, F>(self, f: F) -> State<R>
    where
        F: Fn(State<Q>) -> R,
    {
        State(f(self))
    }
    /// [State::map_mut] applies a [`FnMut`] closure to the state, returing the transformed state.
    pub fn map_mut<R, F>(mut self, f: &mut F) -> State<R>
    where
        F: FnMut(&mut Q) -> R,
    {
        State(f(self.as_mut()))
    }
    /// Maps the state to a new state using a closure that takes the state by value.
    pub fn map_once<R, F>(self, f: F) -> State<R>
    where
        F: FnOnce(State<Q>) -> R,
    {
        State(f(self))
    }
    /// Returns a state with an owned inner value.
    pub fn to_owned(&self) -> State<Q>
    where
        Q: Clone,
    {
        State(self.0.clone())
    }
    /// Returns a shared reference to the state.
    pub fn to_shared(&self) -> State<Arc<Q>>
    where
        Q: Clone,
    {
        State(Arc::new(self.0.clone()))
    }
    /// Returns a state with a mutable reference to the inner value.
    pub fn to_mut<'a>(&'a mut self) -> State<&'a mut Q> {
        State(&mut self.0)
    }
    /// Returns a state with an owned inner value.
    pub fn to_ref<'a>(&'a self) -> State<&'a Q> {
        State(&self.0)
    }

    /// Returns the `name` of the generic inner type, `Q`.
    pub fn inner_type_name(&self) -> &'static str {
        core::any::type_name::<Q>()
    }
    /// Returns the `type id` of the generic inner type, `Q`.
    pub fn inner_type_id(&self) -> core::any::TypeId
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Q>()
    }
    pub fn is_halt(&self) -> bool
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Self>() == core::any::TypeId::of::<State<Halt<Q>>>()
    }
}

impl<Q> State<Halt<Q>> {
    /// Returns a new instance of [State] with a [Halt] sub-state.
    pub fn halt(Halt(inner): Halt<Q>) -> Self {
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

impl<Q> AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        &self.0
    }
}

impl<Q> AsMut<Q> for State<Q> {
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

impl<Q> PartialEq<Q> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.0 == *other
    }
}

impl<Q> PartialOrd<Q> for State<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<Q> From<Q> for State<Q> {
    fn from(state: Q) -> Self {
        Self(state)
    }
}
