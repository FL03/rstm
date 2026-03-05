/*
    Appellation: machine <module>
    Created At: 2026.03.04:17:48:50
    Contrib: @FL03
*/
#![cfg(all(feature = "macros", feature = "alloc"))]

/// The `turing!` macro facilitates the creation of new [`MovingHead`](crate::MovingHead)
/// instances using familiar syntax
///
/// ```ignore
/// tmh! {
///     #[default_state(initial_state)] // optional
///     rules: {(state, symbol) -> direction(next_state, write_symbol); ...};
/// }
/// ```
///
/// ## Basic Usage
///
/// The following example demonstrates the usage of the macro to create a program using three
/// states `{-1, 0, 1}` and two symbols `{0, 1}`.
///
/// ```rust
/// let mut tmh = rstm_core::tmh! {
///     #[default_state(0isize)] // optional, defaults to <Q>::default()
///     program {
///        (0, 0) -> Right(1, 1),
///     };
/// };
/// // add some input
/// tmh.extend_tape([0, 0, 0, 1, 1, 0, 1, 0 , 0, 1, 1, 1]);
/// // run the machine
/// tmh.run();
/// ```
#[macro_export]
macro_rules! tmh {
    {
        $(#[default_state($ds:expr)])?
        program {$(($state:expr, $symbol:expr) -> $dir:ident($next:expr, $write:expr)),* $(,)?} $(;)?
    } => {
        $crate::MovingHead::from_program(
             $crate::programs::Program::from_iter(
                $crate::ruleset! [$(($state, $symbol) -> $dir($next, $write)),*]
            ) $(.with_default_state($ds))?
        )
    };
}
