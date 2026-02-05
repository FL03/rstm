/*
    Appellation: stateful <module>
    Created At: 2026.01.14:21:42:44
    Contrib: @FL03
*/
use super::RawState;
use crate::state::State;

/// [`Stateful`] is a trait used to establish an interface for objects that maintain theor own
/// internal state.
pub trait Stateful<Q>
where
    Q: RawState,
{
    /// returns a reference to the current state
    fn state(&self) -> &State<Q>;
}

/*
 ************* Implementations *************
*/
impl<U, Q> Stateful<Q> for U
where
    U: core::borrow::Borrow<State<Q>>,
    Q: RawState,
{
    fn state(&self) -> &State<Q> {
        self.borrow()
    }
}
