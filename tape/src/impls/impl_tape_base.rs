/*
    Appellation: impl_tape_base <module>
    Created At: 2026.01.14:20:37:28
    Contrib: @FL03
*/
use crate::RawData;
use crate::tape_base::TapeBase;

impl<S, T> TapeBase<S, T>
where
    S: RawData<Elem = T>,
{
    /// Constructs a new [`TapeBase`] instance with the provided storage.
    ///
    /// # Arguments
    ///
    /// * `store` - The underlying storage implementing the `RawSpace` trait.
    ///
    /// # Returns
    ///
    /// A new instance of `TapeBase`.
    pub fn new(store: S) -> Self {
        Self { store }
    }
}
