/*
    Appellation: macros <module>
    Created At: 2026.01.11:11:54:31
    Contrib: @FL03
*/
#![cfg(feature = "macros")]
use rstm::ruler;
use rstm::{Direction, Head, Rule, Tail};

#[test]
fn test_ruler_macro() {
    let rule: Rule<u8, char> = ruler![(0, ' ') -> Right(1u8, 'a')];
    // create a new head for a rule within the program
    let head = Head::new(0, ' ');
    let exp = Tail::new(Direction::Right, 1, 'a');
    // validate the composition of the rule
    assert_eq! { rule.head(), &head }
    assert_eq! { rule.tail(), &exp }
}
