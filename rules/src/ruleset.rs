/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[allow(deprecated)]
mod impl_deprecated;
mod impl_instruction_set;

use crate::types::RuleVec;
use rstm_state::{RawState, State};

/// A [`Program`] contains a collection of [`Rule`]s and an optional initial [`State`].
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct Program<Q = String, S = char>
where
    Q: RawState,
{
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) rules: RuleVec<Q, S>,
}
