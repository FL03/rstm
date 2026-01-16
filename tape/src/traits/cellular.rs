/*
    Appellation: cellular <module>
    Created At: 2026.01.14:20:40:22
    Contrib: @FL03
*/

/// The [`RawCell`] trait defines the base interface common to all _cells_ within a tape
pub trait RawCell {
    type Elem;

    /// returns a reference to the element stored in the cell.
    fn get(&self) -> &Self::Elem;
}
/// [`RawCellMut`] extends the [`RawCell`] trait to provide mutable access to the content of
/// the cell.
pub trait RawCellMut: RawCell {
    /// returns a mutable reference to the element stored in the cell.
    fn get_mut(&mut self) -> &mut Self::Elem;
    /// write's the given value into the cell
    fn write(&mut self, value: Self::Elem) {
        *self.get_mut() = value;
    }
}
/*
 ************* Implementations *************
*/

impl<T> RawCell for core::cell::Cell<T>
where
    Self: AsRef<T>,
{
    type Elem = T;

    fn get(&self) -> &Self::Elem {
        self.as_ref()
    }
}
