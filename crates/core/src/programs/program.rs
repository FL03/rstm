/*
    Appellation: program <module>
    Created At: 2025.09.05:04:59:21
    Contrib: @FL03
*/

use crate::Rule;
use rstm_state::{RawState, State};

#[deprecated(since = "0.1.4", note = "Use the `Program` implementation instead")]
/// The [`Program`] implementation is designed to provide a structured representation of a set
/// of rules and an optional initial state for a Turing machine or similar computational model.
/// It encapsulates the rules that dictate the machine's behavior and the starting point for
/// its execution.
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct ProgramO<Q = String, A = char>
where
    Q: RawState,
{
    pub(crate) initial_state: State<Q>,
    #[cfg(feature = "alloc")]
    pub(crate) rules: alloc::vec::Vec<Rule<Q, A>>,
}
