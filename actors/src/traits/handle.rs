/*
    Appellation: handle <module>
    Created At: 2025.08.30:00:16:00
    Contrib: @FL03
*/

/// [Handle] describes the step-by-step execution of a program; the trait is generalized
/// with the introduction of a single generic parameter, `T`, capable of sufficiently
/// representing any possible object that may be passed to the [handle] method.
///
/// This notion is particularly useful as it allows us to define the process using an actor,
/// before generically implementing the [Engine] trait for the [Executor] struct. Doing so
/// allows for further abstraction by considering the
pub trait Handle<T> {
    type Output;

    fn handle(&mut self, args: T) -> Self::Output;
}
