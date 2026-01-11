/*
    Appellation: impl_deprecated <module>
    Created At: 2025.08.30:23:58:08
    Contrib: @FL03
*/
use super::Tail;
use rstm_state::RawState;

#[doc(hidden)]
impl<Q, A> Tail<Q, A>
where
    Q: RawState,
{
    #[deprecated(
        since = "0.0.7",
        note = "use `view` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_ref(&self) -> Tail<&Q, &A> {
        self.view()
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view_mut` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_mut(&mut self) -> Tail<&mut Q, &mut A> {
        self.view_mut()
    }
}
