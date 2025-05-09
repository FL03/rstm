/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [State] is a generalized state implementation, representing the state of a system or
/// object.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct State<Q: ?Sized = bool>(pub Q);
