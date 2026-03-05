/*
    Appellation: macros <module>
    Created At: 2026.01.11:11:54:31
    Contrib: @FL03
*/
#![cfg(feature = "macros")]

#[test]
fn test_macro_fsm() {
    let fsm = rstm::fsm! {
        default_state: 0;
        rules: {
            (0, ' ') -> Right(1u8, 'a'),
            (1, 'a') -> Left(0u8, ' '),
        }
    };
    assert! { fsm.has_program() }
}
