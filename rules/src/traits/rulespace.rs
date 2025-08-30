/*
    Appellation: rulespace <module>
    Created At: 2025.08.30:08:03:10
    Contrib: @FL03
*/
use crate::types::{Head, Tail};
use rstm_state::RawState;

pub trait RawSpace {
    private! {}
}

pub trait RuleSpace<Q, S>: RawSpace
where
    Q: RawState,
{
    fn get(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq;
}
