/*
    Appellation: halting <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{Haltable, RawState, State};

mod impl_enum;

mod impl_halt;
#[allow(deprecated)]
mod impl_halt_deprecated;
mod impl_halt_ops;
mod impl_halt_repr;

/// [`Halt`] is a generic wrapper implementing the [`RawState`] trait enabling the haltable
/// state functionality.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Halt<Q: ?Sized = bool>(pub Q);

/// [`Halter`] type wraps the inner state with two variants:
///
/// - `Halt(Q)` for a halted state
/// - `State(Q)` for a continuing state
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
pub enum Halter<Q = usize> {
    Halt(Q),
    State(Q),
}

/*
 ************* Implementations *************
*/

scsys::fmt_wrapper! {
    Halt<Q>(
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        Pointer,
        UpperExp,
        UpperHex,
    )
}
impl<Q> RawState for Halt<Q>
where
    Q: RawState,
{
    seal!();
}

impl<Q> RawState for Halter<Q>
where
    Q: RawState,
{
    seal!();
}

impl<Q> State<Halt<Q>>
where
    Q: RawState,
{
    /// Creates a new instance of a [State] with a halted state.
    pub fn halted(state: Q) -> Self {
        State(Halt(state))
    }
}

impl<Q> Haltable<Q> for State<Halter<Q>>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        true
    }
}
