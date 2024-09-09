/*
    Appellation: impl_repr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::error::Error;
use crate::state::{Halt, State};
use core::mem::MaybeUninit;

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

impl<Q> State<*const Q> {
    /// Creates a new instance of state with a raw pointer to the inner value.
    pub fn from_ptr(ptr: *const Q) -> Self {
        Self(ptr)
    }
}

impl<Q> State<*mut Q> {
    /// Creates a new instance of state with a mutable raw pointer to the inner value.
    pub fn from_mut_ptr(ptr: *mut Q) -> Self {
        Self(ptr)
    }
}

impl<Q> State<MaybeUninit<Q>> {
    /// Creates a new instance of state with an initialized inner value.
    pub fn init(value: Q) -> Self {
        Self(MaybeUninit::new(value))
    }
    /// Creates a new instance of state with an uninitialized inner value.
    pub const fn uninit() -> Self {
        Self(MaybeUninit::uninit())
    }
    /// Converts the state into a new instance of [State] with an initialized state.
    pub fn assume_init(self) -> State<Q> {
        State(unsafe { self.get().assume_init() })
    }
    /// Writes a value to the inner state.
    pub fn write(&mut self, value: Q) -> &mut Q {
        self.get_mut().write(value)
    }
}

impl State<()> {
    /// Creates a new instance of [State] with an empty state.
    pub fn empty() -> Self {
        Self(())
    }
}

impl State<bool> {
    pub fn from_true() -> Self {
        Self(true)
    }

    pub fn from_false() -> Self {
        Self(false)
    }

    pub fn is_true(&self) -> bool {
        self.get()
    }

    pub fn is_false(&self) -> bool {
        !self.get()
    }
}

impl State<Box<dyn core::any::Any>> {
    /// Attempts to downcast the state to a concrete type `Q`; returns an error if the state
    /// is not of type `Q`.
    pub fn downcast<Q>(self) -> Result<State<Box<Q>>, Error>
    where
        Q: core::any::Any,
    {
        self.get()
            .downcast()
            .map(State)
            .map_err(|_| Error::type_error("Failed to downcast state"))
    }
    /// Returns an immutable reference to the state if it is of type `Q`; returns `None`
    /// otherwise.
    pub fn downcast_ref<Q>(&self) -> Option<State<&Q>>
    where
        Q: core::any::Any,
    {
        self.get_ref().downcast_ref().map(State)
    }

    /// Returns a mutable reference to the state if it is of type `Q`; returns `None`
    /// otherwise.
    pub fn downcast_mut<Q>(&mut self) -> Option<State<&mut Q>>
    where
        Q: core::any::Any,
    {
        self.get_mut().downcast_mut().map(State)
    }
}

impl<Q> State<Option<Q>> {
    /// Creates a new instance of state whose inner state is [Option::None].
    pub fn none() -> Self {
        Self(None)
    }
    /// Creates a new instance of state whose inner state is [Option::Some].
    pub fn some(value: Q) -> Self {
        Self(Some(value))
    }
}

impl<Q> State<Halt<Q>> {
    /// Creates a new instance of [State] from a [Halt] state.
    pub fn halted(Halt(inner): Halt<Q>) -> Self {
        Self(Halt(inner))
    }
    /// Converts the halted state into an unhalted state.
    pub fn unhalt(self) -> State<Q> {
        State(self.get().get())
    }
}
