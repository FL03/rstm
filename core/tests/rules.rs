/*
    Appellation: rules <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm_core as rstm;

use rstm::rules::RuleSet;
use rstm::{Direction, Head, State, Tail, ruleset};

#[test]
fn ruleset() {
    let rules: RuleSet<isize, usize> = ruleset![
        (0, 0) -> Right(1, 1),
        (0, 1) -> Left(-1, 0),
        (1, 0) -> Right(1, 1),
        (1, 1) -> Left(-1, 1),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Left(0, 1),
    ];
    // validate the number of rules within the ruleset
    assert_eq!(rules.len(), 6);
    // create a new head for a rule within the program
    let head = Head::new(State(0), 0);
    // retrieve and validate the tail for the given head
    assert_eq!(
        rules.get_by_head(&head),
        Some(&Tail::new(Direction::Right, State(1), 1))
    )
}
