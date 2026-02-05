/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::RawState;

/// The [`State`] wrapper is a generic container used to denote objects being used as a
/// _state_ in a Turing machine, finite-state machine, or similar computational model.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct State<Q: ?Sized = bool>(pub Q);

/// The [`Halt`] implementation is a binary enum designed to represent either a halting state
/// or a stepping state within a Turing machine or similar computational model.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::EnumCount, strum::EnumIs,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
pub enum Halt<Q = usize, H = Q>
where
    Q: RawState,
    H: RawState,
{
    Halt(H),
    Step(Q),
}

#[cfg(test)]
mod tests {}
