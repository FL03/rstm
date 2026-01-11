/*
    Appellation: ruleset <module>
    Contrib: @FL03
*/

/// the [`state!`] macro is a simple helper macro to create a [`State`](crate::state::State)
/// instance.
#[macro_export]
macro_rules! state {
    ($state:expr) => {
        $crate::state::State::new($state)
    };
}

#[macro_export]
macro_rules! head {
    ($state:expr, $symbol:expr) => {
        $crate::head::Head {
            state: $crate::state::State::new($state),
            symbol: $symbol,
        }
    };
}

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
/// \delta : Q\times{A}\rightarrow{(Q\times{A})\times{D}\quad\text{where} \ D\in\lbrace{0,\pm{1}\rbrace}
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
/// ```rust

/// let a = eryon_rules::rule![(0, 'a') -> Right(1, 'c')];
/// ```

/// ```
#[macro_export]
macro_rules! rule {
    {$(($state:expr, $symbol:literal) -> $direction:ident($next:expr, $write:literal)),* $(,)?} => {
        $($crate::rule! { @impl ($state, $symbol) -> $direction($next, $write) } )*
    };
    {@impl ($state:expr, $symbol:literal) -> $direction:ident($next:expr, $write:literal)} => {
        $crate::rules::Rule::init()
            .state($crate::state::State($state))
            .symbol($symbol)
            .write_symbol($write)
            .direction($crate::Direction::$direction)
            .next_state($crate::state::State($next))
            .build()
    };
}
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
/// use eryon_rules::ruleset;
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
    {$(
        ($state:expr, $symbol:literal $(,)?) -> $dir:ident($next:expr, $write:literal $(,)?)
    ),* $(,)?} => {
        [$(
            $crate::rule! { ($state, $symbol) -> $dir($next, $write) },
        )*]
    };
}

#[cfg(feature = "std")]
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
/// use eryon_rules::rulemap;
///
/// rulemap! {
///     (0, 1) -> Right(0, 1),
///     (0, 0) -> Left(1, 1),
///     (1, 1) -> Right(0, 0),
/// }
/// ```
#[macro_export]
macro_rules! rulemap {
    {
        $(
            ($state:expr, $symbol:literal) -> $direction:ident($next:expr, $write:literal)
        ),* $(,)?
    } => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert(
                    $crate::Head::new($crate::state!($state), $symbol),
                    $crate::Tail::new($crate::Direction::$direction, $crate::state!($next), $write)
                );
            )*
            map
        }
    };
}
