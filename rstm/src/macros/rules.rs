/*
    Appellation: rules <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// The [`rule!`] macro enables the definition of a single, Turing compatible rule using the
/// following syntax:
///
/// ```no_run
///     (state, symbol) -> Direction(next_state, next_symbol)
/// ```
///
/// The syntax is directly ispired by the simplified definition of a Turing machine as a
/// dynamical system, as described in the paper
/// [On the Topological Dynamics of Turing Machines](https://doi.org/10.1016/S0304-3975(96)00025-4)
/// by Petr Kůrka. Specifically,
///
/// ```math
/// \delta : Q\times{A}\rightarrow Q\times{A}\times{\lbrace\pm{1},0\rbrace}
/// ```
///
/// **note:** it is unnecessary for you to import the [`Direction`](crate::Direction) enum, as
/// the macro hygenically imports each of its variants directly based on your usage.
///
/// ## Examples
///
/// ### _Example #1: Basic Usage_
///
/// ```rust
/// // define some rule, a, where when in state 0 and reading symbol 1, it writes symbol 0,
/// // moves the tape head to the right, and transitions to state 1
/// let a = rstm::rule! {
///     (0, 1) -> Right(1, 0)
/// };
/// ```
#[macro_export]
macro_rules! rule {
    (
        ($state:expr, $symbol:literal) -> $direction:ident($next:expr, $write:literal) $(;)?
    ) => {
        $crate::rules::Rule::new()
            .state($crate::state::State($state))
            .symbol($symbol)
            .write_symbol($write)
            .direction($crate::Direction::$direction)
            .next_state($crate::state::State($next))
            .build()
    };
}
/// [`rules!`] is a macro that simplifies the creation of a vector of [`Rules`](crate::Rule).
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
/// use rstm::rules;
///
/// let rule = rules! {
///     (0, 0) -> Right(1, 1);
///     (0, 1,) -> Left(-1, 0);
///     (1, 0) -> Right(1, 1);
///     (1, 1) -> Left(-1, 1);
///     (-1, 0) -> Right(0, 0);
///     (-1, 1) -> Left(0, 1);
/// };
/// ```
#[macro_export]
macro_rules! rules {
    {
        $(
            ($state:expr, $symbol:literal $(,)?) -> $dir:ident($next:expr, $write:literal $(,)?)
        );* $(;)?
    } => {
        [
            $(
                $crate::rule! {
                    ($state, $symbol) -> $dir($next, $write)
                },
            )*
        ]
    };
}

/// a macro to create a [`HashMap`](std::collections::HashMap) of rules for a Turing machine.
/// The macro takes a list of rules in the form of
///
/// ```no_run
///     (state, symbol) -> Direction(next_state, next_symbol)
/// ```
///
/// ## Basic Usage
///
/// ```rust
/// use eryon_core::rulemap;
///
/// rulemap! {
///     (0, 1) -> Right(0, 1);
///     (0, 0) -> Left(1, 1);
///     (1, 1) -> Right(0, 0);
/// }
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! rulemap {
    (
        $(
            ($state:expr, $symbol:expr) -> $dir:ident($next_state:expr, $next_symbol:expr)
        );* $(;)?
    ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert(
                    $crate::Head::new($crate::state::State($state), $symbol),
                    $crate::Tail::new($crate::Direction::$dir, $crate::state::State($next_state), $next_symbol)
                );
            )*
            map
        }
    };
}
