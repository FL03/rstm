/*
    appellation: impl_halt_deprecated <module>
    authors: @FL03
*/
use crate::state::{Halt, RawState};

#[doc(hidden)]
impl<Q> Halt<Q> where Q: RawState {}
