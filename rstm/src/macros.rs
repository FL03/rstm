/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[macro_export]
macro_rules! rule {
    [($state:expr, $symbol:literal $(,)?) -> $direction:ident($next:expr, $write:literal $(,)?) $(,)?] => {
        $crate::rules::Instruction::new()
            .state($state)
            .symbol($symbol)
            .write_symbol($write)
            .direction($crate::Direction::$direction)
            .next_state($next)
            .build()
    };
}

#[macro_export]
macro_rules! ruleset {
    ($($($rule:tt)*);* $(,)?) => {
        vec![$(rule!($($rule)*)),*]
    };
}
