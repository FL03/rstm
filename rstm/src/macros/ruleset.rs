/*
    Appellation: rules <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [`ruleset!`] is a macro that simplifies the creation of a vector of [`Rules`](crate::Rule).
///
/// ### Syntax
///
/// ```ignore
/// ruleset![(state, symbol) -> direction(next_state, write_symbol), ...]
/// ```
///
/// ### Example
///
/// The following example demonstrates the usage of the macro to create a ruleset using three
/// states `{-1, 0, 1}` and two symbols `{0, 1}`.
///
/// ```rust
/// use rstm::ruleset;
///
/// let rule = ruleset![
///     (0, 0) -> Right(1, 1),
///     (0, 1) -> Left(-1, 0),
///     (1, 0) -> Right(1, 1),
///     (1, 1) -> Left(-1, 1),
///     (-1, 0) -> Right(0, 0),
///     (-1, 1) -> Left(0, 1),
/// ];
/// ```
#[macro_export]
macro_rules! ruleset {
    [@base ($state:expr, $symbol:literal) -> $direction:ident($next:expr, $write:literal)] => {
        $crate::rules::Rule::new()
            .state($crate::state!($state))
            .symbol($symbol)
            .write_symbol($write)
            .direction($crate::Direction::$direction)
            .next_state($crate::state!($next))
            .build()
    };
    {$(initial_state($q:expr);)? $(  ($state:expr, $symbol:literal $(,)?) -> $direction:ident($next:expr, $write:literal $(,)?)  ),* $(,)?} => {
        $crate::rules::RuleSet::from_iter([
            $(
                $crate::ruleset![@base ($state, $symbol) -> $direction($next, $write)],
            )*
        ])$(
            .with_initial_state($crate::state!($q))
        )?
    };
}
