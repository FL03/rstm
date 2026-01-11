/*
    Appellation: program <module>
    Created At: 2025.08.30:17:01:39
    Contrib: @FL03
*/
#![cfg(all(feature = "macros", feature = "alloc"))]

/// The [`program!`] macro facilitates the creation of new [`Program`](crate::programs::Program)
/// instances using familiar syntax
///
/// ```ignore
/// program! {
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
/// let rule = rstm::program![
///     #[default_state(0)] // optional
///     rules: {
///         (0, 0) -> Right(1, 1),
///         (0, 1) -> Left(-1, 0),
///         (1, 0) -> Right(1, 1),
///         (1, 1) -> Left(-1, 1),
///         (-1, 0) -> Right(0, 0),
///         (-1, 1) -> Left(0, 1),
///     };
/// ```
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! program {
    {
        $(#[default_state($q:expr)])?
        rules: {$(($state:expr, $symbol:expr) -> $direction:ident($next:expr, $write:expr)),* $(,)?} $(;)?
    } => {
        $crate::program::Program::from_iter(
            $crate::ruleset! [$(($state, $symbol) -> $direction($next, $write)),*]
        )
        $(.with_default_state($q))?
    };
}
