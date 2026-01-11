/*
    Appellation: ruliad <module>
    Created At: 2025.09.04:19:05:43
    Contrib: @FL03
*/

mod impl_ruliad;

use crate::types::RuleVec;
use rstm_state::RawState;

/// A [`Ruliad`] defines an abstract space consisting of interconnected instructions detailing
/// how a particular system should execute some program;
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct Ruliad<Q = String, A = char>
where
    Q: RawState,
{
    pub(crate) rules: RuleVec<Q, A>,
}
