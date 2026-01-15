/*
    Appellation: ruleset <module>
    Contrib: @FL03
*/
/// The [`head!`] macro simplifies the creation of a [`Head`](crate::Head) struct
/// by providing a concise syntax for specifying the state and symbol.
#[macro_export]
macro_rules! head {
    ($state:expr, $symbol:expr) => {
        $crate::Head {
            state: $crate::State($state),
            symbol: $symbol,
        }
    };
}
///! the [`tail!`] macro facilitates the creation of a [`Tail`](crate::Tail) struct
///! by providing a concise syntax for specifying the direction, next state, and symbol to
/// write. As with other macros in this module, the variants of the [`Direction`](crate::Direction)
/// are hygenically imported based on usage, meaning one only needs to specify the variant name
///! directly, i.e. `Left`, `Right`, or `Stay`.
#[macro_export]
macro_rules! tail {
    ($direction:ident, $state:expr, $symbol:expr) => {
        $crate::Tail {
            direction: $crate::Direction::$direction,
            next_state: $crate::State($state),
            write_symbol: $symbol,
        }
    };
}

/// The [`rule!`] macro enables the definition of a single, Turing compatible rule using the
/// following syntax:
///
/// ```ignore
///     (state, symbol) -> Direction(next_state, next_symbol)
/// ```
///
/// The syntax is directly inspired by the simplified definition of a Turing machine as a
/// dynamical system, as described in the paper [On the Topological Dynamics of Turing Machines](https://doi.org/10.1016/S0304-3975(96)00025-4)
/// by Petr Kůrka. Specifically,
///
/// ```math
/// \delta:Q\times{A}\rightarrow{Q}\times{A}\times\lbrace{0,\pm{1}\rbrace}
/// ```
///
/// **note:** it is unnecessary for you to import the [`Direction`](crate::Direction) enum, as
/// the macro hygenically imports each of its variants directly based on your usage.
///
/// ### _Basic Usage_
///
/// Let's use the macro to define some rule, `a`, that considers a head that is in a state of
/// `0` reading some value `'a'` from the tape, and then writes a `c` to the tape, moves right,
/// and updates the state to `1`.
///
/// ```rust
/// let rule = rstm_core::rule![(0, 'a') -> Right(1, 'c')];
/// ```
#[macro_export]
macro_rules! rule {
    [($state:expr, $symbol:literal) -> $direction:ident($next_state:expr, $write_symbol:literal)] => {
        $crate::Rule {
            head: $crate::head!($state, $symbol),
            tail: $crate::tail!($direction, $next_state, $write_symbol),
        }
    };
}
/// [`ruleset!`] is a macro that simplifies the creation of an array of [`Rules`](crate::Rule).
///
/// The syntax follows the [`rule!`] macro, allowing for multiple rules to be defined in a
/// single invocation.
///
/// ```ignore
/// ruleset![(state, symbol) -> direction(next_state, write_symbol), ...]
/// ```
///
/// ## Basic Usage
///
/// The following example demonstrates the usage of the macro to create a ruleset using three
/// states `{-1, 0, 1}` and two symbols `{0, 1}`.
///
/// ```rust
/// let rule = rstm_core::ruleset![
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
    {$(($state:expr, $symbol:literal $(,)?) -> $dir:ident($next:expr, $write:literal $(,)?)),* $(,)?} => {
        [$($crate::rule! { ($state, $symbol) -> $dir($next, $write) }),*]
    };
}

#[cfg(any(feature = "hashbrown", feature = "std"))]
/// a macro to create a `HashMap` of rules for a Turing machine.
/// The macro takes a list of rules in the form of
///
/// ```ignore
///     (state, symbol) -> Direction(next_state, next_symbol)
/// ```
///
/// ## Basic Usage
///
/// ```rust
/// let rules = rstm_core::head_map! {
///     (0, 1) -> Right(0, 1),
///     (0, 0) -> Left(1, 1),
///     (1, 1) -> Right(0, 0),
/// };
/// ```
#[macro_export]
macro_rules! head_map {
    {$(($state:expr, $symbol:literal) -> $direction:ident($next:expr, $write:literal)),* $(,)?} => {
        {
            let mut map = $crate::HeadMap::new();
            $(
                map.insert(
                    $crate::head!($state, $symbol),
                    $crate::tail!($direction, $next, $write)
                );
            )*
            map
        }
    };
}
