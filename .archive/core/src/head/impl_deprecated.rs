/*
    Appellation: impl_deprecated <module>
    Created At: 2025.08.31:00:00:24
    Contrib: @FL03
*/
use super::Head;
use rstm_state::RawState;

#[doc(hidden)]
impl<Q, S> Head<Q, S>
where
    Q: RawState,
{
    #[deprecated(
        since = "0.0.7",
        note = "use `view` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_ref(&self) -> Head<&Q, &S> {
        Head {
            state: self.state.view(),
            symbol: &self.symbol,
        }
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view_mut` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_mut(&mut self) -> Head<&mut Q, &mut S> {
        self.view_mut()
    }
}
