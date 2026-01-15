/*
    Appellation: cell <module>
    Created At: 2026.01.14:20:43:50
    Contrib: @FL03
*/

/// The [`Cell`] struct is used to define a single slot of memory with a tape-based
/// architecture.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct Cell<T> {
    pub(crate) value: T,
}

pub enum CellEntry<O, V> {
    Occupied(O),
    Vacant(V),
}
