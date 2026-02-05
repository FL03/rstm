/*
    Appellation: wrap <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Halt, RawState, State};

impl<Q, H> RawState for Halt<Q, H>
where
    Q: RawState,
    H: RawState,
{
    seal! {}
}

impl<Q, H> Halt<Q, H>
where
    Q: RawState,
    H: RawState,
{
    /// Creates a new instance of a [Halt] with a halted state.
    pub const fn from_halt(state: H) -> Self {
        Self::Halt(state)
    }
    /// Creates a new instance of a [Halt] with a continuing state.
    pub const fn from_state(state: Q) -> Self {
        Self::Step(state)
    }
    /// [`swap`](core::mem::swap) the inner value of the halt state with that of the given state.
    pub const fn swap(&mut self, other: &mut Halt<Q, H>) {
        match (self, other) {
            (Self::Step(a), Self::Step(b)) => core::mem::swap(a, b),
            (Self::Halt(a), Self::Halt(b)) => core::mem::swap(a, b),
            _ => {}
        }
    }
    /// returns a new instance of the halt state containing a reference to its inner value.
    pub const fn view(&self) -> Halt<&Q, &H> {
        match self {
            Self::Step(inner) => Halt::Step(inner),
            Self::Halt(inner) => Halt::Halt(inner),
        }
    }
    /// returns a new instance of the halt state containing a mutable reference to its inner
    /// value.
    pub const fn view_mut(&mut self) -> Halt<&mut Q, &mut H> {
        match self {
            Self::Step(inner) => Halt::Step(inner),
            Self::Halt(inner) => Halt::Halt(inner),
        }
    }
    /// returns an owned version of the current haltable state
    pub fn to_owned(&self) -> Halt<Q, H>
    where
        Q: Clone,
        H: Clone,
    {
        match self {
            Self::Step(inner) => Halt::Step(inner.clone()),
            Self::Halt(inner) => Halt::Halt(inner.clone()),
        }
    }
}

impl<Q> Halt<Q, Q>
where
    Q: RawState,
{
    #[inline]
    /// consumes the current haltable state, returning the inner state.
    pub fn into_inner(self) -> Q {
        match self {
            Self::Step(inner) => inner,
            Self::Halt(inner) => inner,
        }
    }
    #[inline]
    /// consumes the current instance to ensure a halting state
    pub fn halt(self) -> Halt<Q, Q> {
        match self {
            Self::Step(inner) => Halt::Halt(inner),
            _ => self,
        }
    }
    #[inline]
    /// consumes the current haltable state, returning the inner state.
    pub fn into_state(self) -> State<Q> {
        match self {
            Self::Step(inner) => State(inner),
            Self::Halt(inner) => State(inner),
        }
    }
    #[inline]
    /// consumes the current instance to initialize a wrapper instance
    pub fn into_halt_state(self) -> State<Halt<Q>> {
        State(self)
    }
    /// returns a wrapped instance of the halt state containing a reference to its inner value.
    pub const fn as_halt_state(&self) -> State<Halt<&Q>> {
        State(self.view())
    }
    /// returns a wrapped instance of the halt state containing a mutable reference to its
    /// inner value.
    pub const fn as_mut_halt_state(&mut self) -> State<Halt<&mut Q>> {
        State(self.view_mut())
    }
    /// returns a wrapped reference to the inner value
    pub const fn as_state(&self) -> State<&Q> {
        State(self.get())
    }
    /// returns a wrapped mutable reference to the inner value
    pub const fn as_mut_state(&mut self) -> State<&mut Q> {
        State(self.get_mut())
    }
    /// returns a reference to the internal state
    pub const fn get(&self) -> &Q {
        match self {
            Self::Step(inner) => inner,
            Self::Halt(inner) => inner,
        }
    }
    /// returns a mutable reference to the internal state
    pub const fn get_mut(&mut self) -> &mut Q {
        match self {
            Self::Step(inner) => inner,
            Self::Halt(inner) => inner,
        }
    }
    /// [`replace`](core::mem::replace) the inner value of the halt state with the given state.
    pub const fn replace(&mut self, state: Q) -> Q {
        core::mem::replace(self.get_mut(), state)
    }
    /// update the current inner state without affecting the _status_ of the state.
    pub fn set(&mut self, state: Q) {
        *self.get_mut() = state;
    }
}

impl<'a, Q, H> Halt<&'a Q, &'a H>
where
    Q: RawState,
    H: RawState,
{
    #[inline]
    /// consumes the current instance of the halt state to create another with cloned inner
    /// values.
    pub fn cloned(self) -> Halt<Q, H>
    where
        Q: Clone,
        H: Clone,
    {
        match self {
            Self::Step(inner) => Halt::Step(inner.clone()),
            Self::Halt(inner) => Halt::Halt(inner.clone()),
        }
    }
    #[inline]
    /// consumes the current instance of the halt state to create another with copied inner
    /// values.
    ///
    /// **Note**: This method does not mutate the specified _variant_.
    pub fn copied(self) -> Halt<Q, H>
    where
        Q: Copy,
        H: Copy,
    {
        match self {
            Self::Step(&inner) => Halt::Step(inner),
            Self::Halt(&inner) => Halt::Halt(inner),
        }
    }
}

impl<'a, Q, H> Halt<&'a mut Q, &'a mut H>
where
    Q: RawState,
    H: RawState,
{
    #[inline]
    /// consumes the current instance of the halt state to create another with cloned inner
    /// values.
    pub fn cloned(self) -> Halt<Q, H>
    where
        Q: Clone,
        H: Clone,
    {
        match self {
            Self::Step(inner) => Halt::Step(inner.clone()),
            Self::Halt(inner) => Halt::Halt(inner.clone()),
        }
    }
    #[inline]
    /// consumes the current instance of the halt state to create another with copied inner
    /// values.
    pub fn copied(self) -> Halt<Q, H>
    where
        Q: Copy,
        H: Copy,
    {
        match self {
            Self::Step(&mut inner) => Halt::Step(inner),
            Self::Halt(&mut inner) => Halt::Halt(inner),
        }
    }
}

impl<Q> AsRef<Q> for Halt<Q>
where
    Q: RawState,
{
    fn as_ref(&self) -> &Q {
        self.get()
    }
}

impl<Q> AsMut<Q> for Halt<Q>
where
    Q: RawState,
{
    fn as_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::ops::Deref for Halt<Q>
where
    Q: RawState,
{
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<Q> core::ops::DerefMut for Halt<Q>
where
    Q: RawState,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<Q> Default for Halt<Q>
where
    Q: Default + RawState,
{
    fn default() -> Self {
        Self::Step(Default::default())
    }
}

impl<Q> From<State<Q>> for Halt<Q>
where
    Q: RawState,
{
    fn from(State(state): State<Q>) -> Self {
        Self::Step(state)
    }
}
