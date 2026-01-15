/*
    Appellation: stated <module>
    Created At: 2026.01.14:21:42:44
    Contrib: @FL03
*/
#![cfg(feature = "nightly")]
use super::RawState;
use crate::state::State;

pub trait Stated<Q>
where
    Q: RawState,
{
    fn stated(self: State<Self>) -> State<Q>;
}
