/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{Halt, State};

/// [TMS] extends the T
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::EnumCount,
    strum::EnumIs,
)]
pub enum TMS<Q> {
    Halt(State<Halt<Q>>),
    State(State<Q>),
}

impl<Q> TMS<Q> {
    pub fn halt(State(state): State<Halt<Q>>) -> Self {
        TMS::Halt(State(state))
    }

    pub fn state(state: State<Q>) -> Self {
        TMS::State(state)
    }

    pub fn into_halt(self) -> State<Halt<Q>> {
        match self {
            TMS::Halt(state) => state,
            TMS::State(state) => State(Halt(state.into_inner())),
        }
    }

    pub fn into_state(self) -> State<Q> {
        match self {
            TMS::Halt(state) => state.unhalt(),
            TMS::State(state) => state,
        }
    }
}
