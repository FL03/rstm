/*
    Appellation: step <module>
    Created At: 2026.01.14:20:53:24
    Contrib: @FL03
*/

/// A trait defining an unary step operation primarily intended to enable iterable execution
/// patterns from implemented machines.
pub trait TryStep {
    type Error;
    type Output;

    /// Performs a step operation
    fn try_step(&mut self) -> Result<Self::Output, Self::Error>;
}
