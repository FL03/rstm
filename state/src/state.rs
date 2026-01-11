/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

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

/// Inline with the formal definition of a halt state, the [`Halter`] implementation
/// extends a stateful object, enabling _**any**_ given state to be halted.
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
#[repr(C)]
pub enum Halter<Q = usize, H = Q> {
    Halt(H),
    Step(Q),
}

#[cfg(test)]
mod tests {}
