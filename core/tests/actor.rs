/*
    Appellation: actor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm_core as rstm;

use rstm::actors::Actor;
use rstm::rules::{Program, Rule};
use rstm::{ruleset, State};

const INITIAL_STATE: State<isize> = State(0);

lazy_static::lazy_static! {
    static ref RULES: [Rule<isize, usize>; 6] = ruleset![
        (0, 0) -> Right(1, 1),
        (0, 1) -> Left(-1, 0),
        (1, 0) -> Right(1, 1),
        (1, 1) -> Left(-1, 1),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Left(0, 1),
    ];

    static ref PROGRAM: Program<isize, usize> = Program::from_iter(RULES.clone()).with_initial_state(INITIAL_STATE);
}

#[test]
fn busy_beaver() {
    let input = [0_usize; 10];

    let program = Program::new()
        .rules(RULES.clone())
        .initial_state(INITIAL_STATE)
        .build();

    let actor = Actor::new(State(0)).with_tape(input);
    assert!(actor.execute(program).run().is_ok());
}
