/*
    appellation: halted <module>
    authors: @FL03
*/
use crate::traits::RawState;

/// The [`Haltable`] trait defines a contract for types that can be checked for a halted state.
pub trait Haltable<Q>
where
    Q: RawState,
{
    private!();

    fn is_halted(&self) -> bool;
}

/*
 ************* Implementations *************
*/
use crate::state::State;
use crate::types::Halt;

impl<Q> Haltable<Q> for Option<Q>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

impl<Q> Haltable<Q> for State<Option<Q>>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.0.is_none()
    }
}

impl<Q> Haltable<Q> for Option<State<Q>>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

impl<Q> Haltable<Q> for Halt<Q>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        matches!(self, Halt::Halt(_))
    }
}

impl<Q> Haltable<Q> for &Halt<Q>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        matches!(self, Halt::Halt(_))
    }
}

impl<Q> Haltable<Q> for State<Q>
where
    Q: RawState + Haltable<Q>,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.0.is_halted()
    }
}