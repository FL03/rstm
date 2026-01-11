/*
    Appellation: macros <module>
    Created At: 2026.01.11:11:54:31
    Contrib: @FL03
*/
#![cfg(feature = "macros")]
use rstm::ruler;
use rstm::{Direction, Head, Tail};

#[test]
fn test_ruler_macro() {
    let rule = ruler![(0u8, 'a') -> Right(1u8, 'b')];
    // create a new head for a rule within the program
    let head = Head::new(0, 0);
    let exp = Tail::new(Direction::Right, 1, 1);
    // validate the composition of the rule
    assert_eq! { rule.head(), &head }
    assert_eq! { rule.tail(), &exp }
}
