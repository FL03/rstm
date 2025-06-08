/*
    Appellation: ruleset <module>
    Contrib: @FL03
*/

/// a macro to create a [`HashMap`](std::collections::HashMap) of rules for a Turing machine.
/// The macro takes a list of rules in the form of
///
/// ```no_run
///     (state, symbol) -> Direction(next_state, next_symbol)
/// ```
///
///
/// ## Basic Usage
///
/// ```rust
/// use eryon_core::rulemap;
///
/// rulemap! {
///     (0, 1) -> Right(0, 1),
///     (0, 0) -> Left(1, 1),
///     (1, 1) -> Right(0, 0),
/// }
/// ```
#[cfg(all(feature = "macros", feature = "std"))]
#[macro_export]
macro_rules! rulemap {
    ($(($state:expr, $symbol:expr) -> $dir:ident($next_state:expr, $next_symbol:expr)),* $(,)?) => {
        {
            let mut map = $crate::rules::RuleMap::new();
            $(
                map.insert(
                    $crate::Head::new($crate::State($state), $symbol),
                    $crate::Tail::new($crate::Direction::$dir, $crate::State($next_state), $next_symbol)
                );
            )*
            map
        }
    };
}
