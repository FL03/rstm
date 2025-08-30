/*
    Appellation: rule <module>
    Created At: 2025.08.30:01:04:10
    Contrib: @FL03
*/

mod impl_rule;
mod impl_rule_builder;
mod impl_rule_ext;

use rstm_core::{Direction, Head, Tail};
use rstm_state::{RawState, State};

/// The [`Rule`] defines the core structure of a single instruction within a viable rulespace.
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

#[derive(Default)]
pub struct RuleBuilder<Q, S> {
    direction: Direction,
    state: Option<State<Q>>,
    symbol: Option<S>,
    next_state: Option<State<Q>>,
    write_symbol: Option<S>,
}
