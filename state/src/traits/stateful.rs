/*
    Appellation: stateful <module>
    Created At: 2025.08.31:08:47:19
    Contrib: @FL03
*/
use crate::RawState;

/// The [`Stateful`] trait defines an interface for any objects that contain an explicit state.
pub trait Stateful<U>
where
    U: RawState,
{
    type State<_V: RawState>;

    fn state(&self) -> Self::State<U>;
}
