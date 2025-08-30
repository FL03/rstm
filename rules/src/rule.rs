/*
    Appellation: rule <module>
    Created At: 2025.08.30:01:04:10
    Contrib: @FL03
*/
mod impl_learned_rule;

mod impl_rule;
mod impl_rule_builder;
mod impl_rule_ext;

use rstm_core::{Direction, Head, Tail};
use rstm_state::{RawState, State};

/// The [`Rule`] implementation is a concrete representation of a single instruction, or rule, 
/// within a given Turing machine program. It encapsulates the necessary components to define 
/// the behavior of the Turing machine when it encounters a specific state and symbol.
/// 
/// **Note**: The inner fields are flattened for serialization purposes when using `serde`; 
/// this means that the fields of the `Head` and `Tail` structs will be serialized as if they 
/// were direct fields of the `Rule` struct itself.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "camelCase")
)]
pub struct Rule<Q = String, A = char>
where
    Q: RawState,
{
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub head: Head<Q, A>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub tail: Tail<Q, A>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LearnedRule<C = f32, Q = usize, S = usize>
where
    Q: RawState,
{
    pub confidence: C,
    pub head: Head<Q, S>,
    pub tail: Tail<Q, S>,
}

#[derive(Default)]
pub struct RuleBuilder<Q, S> {
    direction: Direction,
    state: Option<State<Q>>,
    symbol: Option<S>,
    next_state: Option<State<Q>>,
    write_symbol: Option<S>,
}