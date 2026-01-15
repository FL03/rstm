/*
    Appellation: rule <module>
    Created At: 2025.08.30:01:04:10
    Contrib: @FL03
*/
use crate::{Direction, Head, Tail};
use rstm_state::{RawState, State};

/// The [`Rule`] implementation is a concrete representation of a single instruction, or rule,
/// within a given Turing machine program. It encapsulates the necessary components to define
/// the behavior of the Turing machine when it encounters a specific state and symbol.
///
/// **Note**: The inner fields are flattened for serialization purposes when using `serde`;
/// this means that the fields of the [`Head`] and [`Tail`] structs will be serialized as if they
/// were direct fields of the `Rule` struct itself.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct Rule<Q = String, A = char>
where
    Q: RawState,
{
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub head: Head<Q, A>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub tail: Tail<Q, A>,
}

/// A [`LearnedRule`] is an extension of the basic [`Rule`] structure, incorporating a
/// confidence metric to quantify the reliability or certainty of the rule within the scope of
/// a learning context. This is particularly useful in scenarios where rules are derived from
/// data or experience, allowing for a more nuanced application of rules based on their
/// confidence levels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct LearnedRule<C = f32, Q = usize, S = usize>
where
    Q: RawState,
{
    pub confidence: C,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub rule: Rule<Q, S>,
}

#[derive(Default)]
pub struct RuleBuilder<Q, S>
where
    Q: RawState,
{
    pub(crate) direction: Direction,
    pub(crate) state: Option<State<Q>>,
    pub(crate) symbol: Option<S>,
    pub(crate) next_state: Option<State<Q>>,
    pub(crate) write_symbol: Option<S>,
}

#[cfg(test)]
mod tests {}
