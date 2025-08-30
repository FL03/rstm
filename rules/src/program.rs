/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[allow(deprecated)]
mod impl_deprecated;
mod impl_program;

use crate::types::RuleVec;
use rstm_state::{RawState, State};

/// A [`Program`] defines an optional initial state along with a set of rules that dictate the
/// behavior of the system.
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
