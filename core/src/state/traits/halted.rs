/*
    appellation: halted <module>
    authors: @FL03
*/
use crate::state::{RawState, State};

pub trait Haltable<Q>
where
    Q: RawState,
{
    private!();

    fn is_halted(&self) -> bool;
}

#[doc(hidden)]
pub trait HaltableExt<Q>: Haltable<Q>
where
    Q: RawState,
{
    fn get(self) -> Option<Q>;

    fn get_mut(&mut self) -> Option<&mut Q>;

    fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(Q) -> U,
        Self: Sized,
    {
        self.get().map(f)
    }
}

/*
 ************* Implementations *************
*/
use core::ops::ControlFlow;
impl<Q> Haltable<Q> for State<ControlFlow<Q, Q>>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        matches!(self.0, ControlFlow::Break(_))
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

impl<Q> HaltableExt<Q> for Option<State<Q>>
where
    Q: RawState,
{
    fn get(self) -> Option<Q> {
        self.map(|state| state.value())
    }

    fn get_mut(&mut self) -> Option<&mut Q> {
        self.as_mut().map(|state| state.get_mut())
    }
}

impl<Q> HaltableExt<Q> for State<Option<Q>>
where
    Q: RawState,
{
    fn get(self) -> Option<Q> {
        self.0
    }

    fn get_mut(&mut self) -> Option<&mut Q> {
        self.0.as_mut()
    }
}
