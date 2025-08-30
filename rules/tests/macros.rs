/*
    Appellation: rules <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstm_core::Direction;
use rstm_rules::{Head, Tail};
use rstm_state::State;

#[test]
fn test_ruleset() {
    use rstm_rules::program;

    let rules = program![
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
        rules.get(&head),
        Some(&Tail::new(Direction::Right, State(1), 1))
    )
}

#[cfg(feature = "std")]
#[test]
fn test_rulemap() {
    use rstm_rules::rulemap;
    // create a new ruleset using the macro
    let rules = rulemap! {
        (0, 0) -> Right(1, 1),
        (0, 1) -> Left(-1, 0),
        (1, 0) -> Right(1, 1),
        (1, 1) -> Left(-1, 1),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Left(0, 1),
    };
    // validate the number of rules within the ruleset
    assert_eq!(rules.len(), 6);
    // create a new head for a rule within the program
    let head = Head::new(State(0), 0);
    // retrieve and validate the tail for the given head
    assert_eq!(
        rules.get(&head),
        Some(&Tail::new(Direction::Right, State(1), 1))
    )
}
