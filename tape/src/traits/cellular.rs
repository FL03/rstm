/*
    Appellation: cellular <module>
    Created At: 2026.01.14:20:40:22
    Contrib: @FL03
*/

pub trait RawCell {
    type Elem;

    /// Reads the value from the cell.
    fn read(&self) -> Self::Elem;

    /// Writes a value to the cell.
    fn write(&mut self, value: Self::Elem);
}

pub trait RawTape<T> {
    type Cell<V>: RawCell<Elem = V>;

    /// Returns the length of the tape.
    fn len(&self) -> usize;

    /// Returns true if the tape is empty.
    fn is_empty(&self) -> bool;

    /// Gets a reference to a cell at the specified position.
    fn get(&self, position: usize) -> Option<&Self::Cell<T>>;

    /// Gets a mutable reference to a cell at the specified position.
    fn get_mut(&mut self, position: usize) -> Option<&mut Self::Cell<T>>;
}
