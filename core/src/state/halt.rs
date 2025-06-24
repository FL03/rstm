/*
    Appellation: halting <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{Haltable, RawState, State};

mod impl_enum;

/// [`Halt`] type wraps the inner state with two variants:
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
pub enum Halt<Q = usize> {
    Halt(Q),
    State(Q),
}

/*
 ************* Implementations *************
*/
impl<Q> RawState for Halt<Q>
where
    Q: RawState,
{
    seal!();
}

impl<Q> Haltable<Q> for State<Halt<Q>>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        true
    }
}
