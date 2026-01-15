/*
    Appellation: tape_base <module>
    Created At: 2026.01.14:20:35:34
    Contrib: @FL03
*/
use crate::RawData;

/// [`TapeBase`] provides a generic implementation of a linear, cell-based memory system that
/// can be used in various computational contexts.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct TapeBase<S, A = <S as RawData>::Elem>
where
    S: RawData<Elem = A>,
{
    pub(crate) store: S,
}
