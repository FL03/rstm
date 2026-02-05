/*
    Appellation: cell <module>
    Created At: 2026.01.14:20:43:50
    Contrib: @FL03
*/
/// An alias for a [`CellBase`] instance containing a `core::cell::Cell<T>`.
pub type SuperCell<T> = CellBase<core::cell::Cell<T>>;
/// An alias for a [`CellBase`] instance containing a reference to `T`.
pub type CellRef<'a, T> = CellBase<&'a T>;
/// An alias for a [`CellBase`] instance containing a mutable reference to `T`.
pub type CellMut<'a, T> = CellBase<&'a mut T>;
#[cfg(feature = "alloc")]
/// A type alias for a [`CellBase`] instance using the `alloc::boxed::Box` smart pointer.
pub type BoxedCell<T> = CellBase<alloc::boxed::Box<T>>;
#[cfg(feature = "alloc")]
/// A type alias for a [`CellBase`] instance using the `alloc::rc::Rc` smart pointer.
pub type RcCell<T> = CellBase<alloc::rc::Rc<T>>;
#[cfg(feature = "alloc")]
/// A type alias for a [`CellBase`] containing an [`Arc`](alloc::sync::Arc) smart pointer.
pub type SharedCell<T> = CellBase<alloc::sync::Arc<T>>;

/// The [`CellBase`] struct is used to define a single slot of memory with a tape-based
/// architecture.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct CellBase<T> {
    pub(crate) value: T,
}

pub enum CellEntry<O, V> {
    Occupied(O),
    Vacant(V),
}
