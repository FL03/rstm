/*
    Appellation: rules <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm_core as rstm;

use rstm::rules::Program;
use rstm::{ruleset, Direction, Head, State, Tail};

#[test]
fn ruleset() {
    let rules = ruleset![
        (0, 0) -> Right(1, 1),
        (0, 1) -> Left(-1, 0),
        (1, 0) -> Right(1, 1),
        (1, 1) -> Left(-1, 1),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Left(0, 1),
    ];
    // validate the number of rules within the ruleset
    assert_eq!(rules.len(), 6);
    // create a new program using the ruleset
    let program = Program::new()
        .initial_state(State(0isize))
        .rules(rules)
        .build();
    // validate the number of rules within the program
    assert_eq!(rules.len(), program.len());
    // compare the inner value of the initial state
    assert_eq!(program.initial_state(), &0);
    // create a new head for a rule within the program
    let head = Head::new(State(0), 0);
    // retrieve and validate the tail for the given head
    assert_eq!(
        program.get_head(&head),
        Some(&Tail::new(Direction::Right, State(1), 1))
    )
}
