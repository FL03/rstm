/*
    appellation: impl_state_deprecated <module>
    authors: @FL03
*/
use crate::state::{RawState, State};

#[doc(hidden)]
impl<Q> State<Q>
where
    Q: RawState,
{
    #[deprecated(
        since = "0.0.7",
        note = "use `value` instead, as it is more idiomatic and clearer."
    )]
    pub fn into_inner(self) -> Q {
        self.0
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_ref(&self) -> State<&Q> {
        self.view()
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view_mut` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_mut(&mut self) -> State<&mut Q> {
        self.view_mut()
    }
}
