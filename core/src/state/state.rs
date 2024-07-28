/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
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
    /// Transforms the state into its inner value.
    pub fn into_inner(self) -> Q {
        self.0
    }
    /// Transforms the state into a shared reference.
    pub fn into_shared(self) -> State<Arc<Q>> {
        State(Arc::new(self.0))
    }
    /// Returns an immutable reference to the state.
    pub fn as_ref(&self) -> &Q {
        &self.0
    }
    /// Returns a mutable reference to the state.
    pub fn as_mut(&mut self) -> &mut Q {
        &mut self.0
    }
    /// Returns a shared reference to the state.
    pub fn as_shared(&self) -> State<Arc<Q>>
    where
        Q: Clone,
    {
        State(Arc::new(self.0.clone()))
    }
    /// Returns a state with an owned inner value.
    pub fn to_view<'a>(&'a self) -> State<&'a Q> {
        State(&self.0)
    }
    /// Returns a state with a mutable reference to the inner value.
    pub fn to_view_mut<'a>(&'a mut self) -> State<&'a mut Q> {
        State(&mut self.0)
    }
    /// Returns the `name` of the generic inner type, `Q`.
    pub fn state_type_name(&self) -> &'static str {
        core::any::type_name::<Q>()
    }
    /// Returns the `type id` of the generic inner type, `Q`.
    pub fn state_type_id(&self) -> core::any::TypeId
    where
        Q: 'static,
    {
        core::any::TypeId::of::<Q>()
    }

    /// Returns the `type id` of the generic inner type, `Q`.
    pub fn is_halt(&self) -> bool
    where
        Q: 'static,
    {
        self.state_type_id() == core::any::TypeId::of::<super::halting::Halt<Q>>()
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

impl<Q> core::cmp::PartialEq<Q> for State<Q>
where
    Q: core::cmp::PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.0 == *other
    }
}

impl<Q> core::cmp::PartialOrd<Q> for State<Q>
where
    Q: core::cmp::PartialOrd,
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
