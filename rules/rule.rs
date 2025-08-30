/*
    Appellation: instruction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

mod impl_rule;
mod impl_rule_builder;
mod impl_rule_ext;

use crate::Direction;
use crate::rules::{Head, Tail};
use crate::state::{RawState, State};

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
    pub head: Head<Q, A>,
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
