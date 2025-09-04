/*
    Appellation: program <module>
    Created At: 2025.08.30:17:01:39
    Contrib: @FL03
*/
#![cfg(all(feature = "rules", feature = "alloc"))]

/// The [`program!`] macro facilitates the generation of new [`InstructionSet`](crate::rules::InstructionSet)
/// instances using familiar syntax.
///
/// ```ignore
/// program! {
///     #[default_state(initial_state)] // optional
///     inputs: [input1, input2, ...] // optional
///     rules: [(state, symbol) -> direction(next_state, write_symbol), ...]
/// }
/// ```
///
/// ### Example
///
/// The following example demonstrates the usage of the macro to create a program using three
/// states `{-1, 0, 1}` and two symbols `{0, 1}`.
///
/// ```rust
/// use rstm::program;
///
/// let rule = program![
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
        rules: {$(($state:expr, $symbol:expr) -> $direction:ident($next:expr, $write:expr));* $(;)?};
    } => {
        $crate::rules::Program::from_iter(
            $crate::rules! {
                $(
                    ($state, $symbol) -> $direction($next, $write)
                );*
            }
        )
        $(
            .with_default_state($q)
        )?
    };
}
