/*
    appellation: impl_halt_repr <module>
    authors: @FL03
*/
use crate::state::{Halt, RawState};

impl<Q> Halt<*const Q>
where
    Q: RawState,
{
    /// Creates a new instance of halt with a raw pointer to the inner value.
    pub fn from_ptr(ptr: *const Q) -> Self {
        Self(ptr)
    }
}

impl<Q> Halt<&Q>
where
    Q: RawState,
{
    /// Clones the internal state and returning a new instance of [`Halt`]
    pub fn cloned(&self) -> Halt<Q>
    where
        Q: Clone,
    {
        Halt(self.0.clone())
    }
    /// Copies the internal state and returning a new instance of [`Halt`]
    pub fn copied(&self) -> Halt<Q>
    where
        Q: Copy,
    {
        Halt(*self.0)
    }
}

impl<Q> Halt<&mut Q>
where
    Q: RawState,
{
    /// Clones the internal state and returning a new instance of [`Halt`]
    pub fn cloned(&self) -> Halt<Q>
    where
        Q: Clone,
    {
        Halt(self.0.clone())
    }
    /// Copies the internal state and returning a new instance of [`Halt`]
    pub fn copied(&self) -> Halt<Q>
    where
        Q: Copy,
    {
        Halt(*self.0)
    }
}
