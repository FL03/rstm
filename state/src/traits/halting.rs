/*
    appellation: halted <module>
    authors: @FL03
*/
use crate::state::State;

/// The [`Haltable`] trait defines a contract for types that can be checked for a halted state.
pub trait Haltable<Q> {
    private!();

    fn is_halted(&self) -> bool;
}

/*
 ************* Implementations *************
*/
impl<Q> Haltable<Q> for State<Option<Q>> {
    seal!();

    fn is_halted(&self) -> bool {
        self.0.is_none()
    }
}

impl<Q> Haltable<Q> for Option<State<Q>> {
    seal!();

    fn is_halted(&self) -> bool {
        self.is_none()
    }
}
