/*
    Appellation: rules <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm_core as rstm;

use rstm::{Head, Tail, rule, ruleset};

#[test]
fn test_rule() {
    // create a new ruleset using the macro
    let rule = rule![(0, 0) -> Right(1, 1)];
    // create a new head for a rule within the program
    let head = Head::new(0, 0);
    let exp = Tail::right(1, 1);
    // validate the composition of the rule
    assert_eq! { rule.head(), &head }
    assert_eq! { rule.tail(), &exp }
}

#[test]
fn test_ruleset() {
    // create a new ruleset using the macro
    let rules = ruleset![
        (0, 0) -> Right(1, 1),
        (0, 1) -> Left(-1, 0),
        (1, 0) -> Right(1, 1),
        (1, 1) -> Left(-1, 1),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Left(0, 1),
    ];
    // ensure each rule is present within the ruleset
    assert_eq! { rules.len(), 6 }
    // validate some rule within the ruleset
    let head = Head::new(0, 0);
    let exp = Tail::right(1, 1);
    assert_eq! { rules[0].head(), &head }
    assert_eq! { rules[0].tail(), &exp }
}
