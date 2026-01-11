/*
    Appellation: convert <module>
    Created At: 2026.01.11:12:43:19
    Contrib: @FL03
*/
use crate::state::State;
use crate::traits::RawState;

/// [`IntoState`] defines a conversion consuming the caller and transforming into a [`State`]
/// of type `Q`.
pub trait IntoState<Q> {
    private! {}

    fn into_state(self) -> State<Q>;
}
/*
 ************* Implementations *************
*/

impl<U, Q> IntoState<Q> for U
where
    Q: RawState,
    U: Into<State<Q>>,
{
    seal! {}

    fn into_state(self) -> State<Q> {
        self.into()
    }
}
