

use crate::cell::Cell;

impl<T> Cell<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }
    /// returns a reference to the value of the cell
    pub const fn get(&self) -> &T {
        &self.value
    }
    /// returns a mutable reference to the value of the cell
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    /// consumes the cell to return the inner value
    pub fn value(self) -> T {
        self.value
    }
    /// [`replace`](core::mem::replace) the value of a cell with the given value, returning the
    /// previous state.
    pub const fn replace(&mut self, value: T) -> T {
        core::mem::replace(self.get_mut(), value)
    }
}