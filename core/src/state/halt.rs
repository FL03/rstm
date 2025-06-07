/*
    Appellation: halting <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{State, Stated};

mod impl_enum;
mod impl_halt;

pub trait Haltable<Q> {
    type State: Stated<Item = Q>;

    private!();

    fn is_halted(&self) -> bool;
}

#[doc(hidden)]
pub trait HaltableExt<Q>: Haltable<Q> {
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Halt<Q>(pub Q);

/// [HaltState] extends the [State] by allowing for an 'imaginary' state that is not actually
/// part of the machine's state space.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[strum_discriminants(name(HaltTag), derive(Hash, Ord, PartialOrd))]
pub enum HaltState<Q = usize> {
    Halt(Halt<Q>),
    State(State<Q>),
}

/*
 ************* Implementations *************
*/
impl<Q> Haltable<Q> for Halt<Q> {
    type State = State<Q>;

    seal!();

    fn is_halted(&self) -> bool {
        true
    }
}

impl<Q> Haltable<Q> for State<Halt<Q>> {
    type State = State<Q>;

    seal!();

    fn is_halted(&self) -> bool {
        true
    }
}
impl<Q> Haltable<Q> for State<Option<Q>> {
    type State = State<Q>;

    seal!();

    fn is_halted(&self) -> bool {
        self.get().is_none()
    }
}

impl<Q> Haltable<Q> for Option<State<Q>> {
    type State = State<Q>;

    seal!();

    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

impl<Q> HaltableExt<Q> for Halt<Q> {
    fn get(self) -> Option<Q> {
        Some(self.0)
    }

    fn get_mut(&mut self) -> Option<&mut Q> {
        Some(&mut self.0)
    }
}

impl<Q> HaltableExt<Q> for Option<State<Q>> {
    fn get(self) -> Option<Q> {
        self.map(|state| state.value())
    }

    fn get_mut(&mut self) -> Option<&mut Q> {
        self.as_mut().map(|state| state.get_mut())
    }
}

impl<Q> HaltableExt<Q> for State<Option<Q>> {
    fn get(self) -> Option<Q> {
        self.value()
    }

    fn get_mut(&mut self) -> Option<&mut Q> {
        self.get_mut().as_mut()
    }
}
