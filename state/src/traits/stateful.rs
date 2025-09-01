/*
    Appellation: stateful <module>
    Created At: 2025.08.31:08:47:19
    Contrib: @FL03
*/
use crate::RawState;

/// The [`StateRepr`] trait defines a common interface for any implemented representations of
/// state.
pub trait StateRepr<U>
where
    U: RawState,
{
    type Repr<_V: RawState>;

    private!();
    /// return a reference to the underlying state
    fn get(&self) -> &U;
    /// return a mutable reference to the underlying state
    fn get_mut(&mut self) -> &mut U;
    /// returns a _view_ of the representation using a reference to the current value
    fn view(&self) -> Self::Repr<&U>;
    /// returns a _view_ of the representation containing a mutable reference to the current
    /// value
    fn view_mut(&mut self) -> Self::Repr<&mut U>;
    /// [`replace`](core::mem::replace) the inner value with another value, returning the old value
    fn replace(&mut self, value: U) -> U {
        core::mem::replace(self.get_mut(), value)
    }
    /// update the inner value
    fn set(&mut self, value: U) {
        *self.get_mut() = value;
    }
    /// [`swap`](core::mem::swap) the inner value with another value
    fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut())
    }
    /// [`take`](core::mem::take) the inner value, leaving a default value in its place
    fn take(&mut self) -> U
    where
        U: Default,
    {
        core::mem::take(self.get_mut())
    }
}
