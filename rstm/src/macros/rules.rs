/*
    Appellation: rules <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// A macro for creating.
///
/// # Example
///
/// ```
///
/// use rstm::rule;
///
/// let rule = rule![(0, '0') -> Right(1, '1')];
#[macro_export]
macro_rules! rule {
    [($state:expr, $symbol:literal $(,)?) -> $direction:ident($next:expr, $write:literal $(,)?) $(,)?] => {
        rstm_core::Instruction::new()
            .state(rstm_core::State($state))
            .symbol($symbol)
            .write_symbol($write)
            .direction(rstm_core::Direction::$direction)
            .next_state(rstm_core::State($next))
            .build()
    };
}
