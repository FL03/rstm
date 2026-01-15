/*
    Appellation: step <module>
    Created At: 2026.01.14:20:53:24
    Contrib: @FL03
*/



pub trait StepOnce<Rhs = Self> {
    type Output;

    /// Performs a single step operation with the given right-hand side operand.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right-hand side operand for the step operation.
    ///
    /// # Returns
    ///
    /// The result of the step operation.
    fn step_once(self, rhs: Rhs) -> Self::Output;
}

pub trait StepMut<Rhs = Self> {
    type Output;

    /// Performs a mutable step operation with the given right-hand side operand.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right-hand side operand for the step operation.
    ///
    /// # Returns
    ///
    /// The result of the step operation.
    fn step_mut(&mut self, rhs: Rhs) -> Self::Output;
}

pub trait Step<Rhs = Self> {
    type Output;

    /// Performs a step operation with the given right-hand side operand.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right-hand side operand for the step operation.
    ///
    /// # Returns
    ///
    /// The result of the step operation.
    fn step(&self, rhs: Rhs) -> Self::Output;
}