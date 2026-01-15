/*
    Appellation: execute <module>
    Contrib: @FL03
*/

/// The [`ExecuteOnce`] defines the ability for an object to perform **exactly** one execution
/// or computation.
pub trait ExecuteOnce<Rhs> {
    type Output;

    fn execute(self, rhs: Rhs) -> Self::Output;
}
/// A trait denoting the ability to execute a given operation, transaction, etc.
pub trait Execute<Rhs> {
    type Output;

    fn execute(&self, rhs: Rhs) -> Self::Output;
}
/// The [`ExecuteMut`] trait is similar to [`Execute`], but it allows for
/// mutable execution of the operation, transaction, etc. This is useful when
/// the operation needs mutable access to the data being operated on.
pub trait ExecuteMut<Rhs> {
    type Output;

    fn execute(&mut self, rhs: Rhs) -> Self::Output;
}

/// The [`TryExecuteOnce`] trait defines the ability for an object to perform
/// one execution or computation that may fail, returning a `Result` configured to use the
/// defined error type.
pub trait TryExecuteOnce<Rhs> {
    type Error;
    type Output;

    fn try_execute(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryExecuteMut<Rhs> {
    type Error;
    type Output;

    fn try_execute(&mut self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryExecute<Rhs> {
    type Error;
    type Output;

    fn try_execute(&self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/*
 ************* Implementations *************
*/
impl<F, U, V> ExecuteOnce<U> for F
where
    F: FnOnce(U) -> V,
{
    type Output = V;

    fn execute(self, rhs: U) -> Self::Output {
        self(rhs)
    }
}

impl<F, U, V> ExecuteMut<U> for F
where
    F: FnMut(U) -> V,
{
    type Output = V;

    fn execute(&mut self, rhs: U) -> Self::Output {
        self(rhs)
    }
}

impl<F, U, V> Execute<U> for F
where
    F: Fn(U) -> V,
{
    type Output = V;

    fn execute(&self, rhs: U) -> Self::Output {
        self(rhs)
    }
}
