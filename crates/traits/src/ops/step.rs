/*
    Appellation: step <module>
    Created At: 2026.01.14:20:53:24
    Contrib: @FL03
*/

/// The [`TryStep`] trait establishes an interface for a fallible step operation
pub trait TryStep {
    type Error;
    type Output;

    /// try to perform a step operation
    fn try_step(&mut self) -> Result<Self::Output, Self::Error>;
}
