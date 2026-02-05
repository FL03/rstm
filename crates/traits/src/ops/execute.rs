/*
    Appellation: execute <module>
    Contrib: @FL03
*/

/// The [`TryExecute`] trait is used to define a fallible execution operation over some `Rhs`.

pub trait TryExecute<Rhs> {
    type Error;
    type Output;

    fn try_execute(&mut self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}
