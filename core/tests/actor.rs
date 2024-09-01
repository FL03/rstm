/*
    Appellation: actor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm_core as rstm;

use rstm::actors::Actor;
use rstm::rules::{Program, Rule};
use rstm::{ruleset, State};

lazy_static::lazy_static! {
    static ref RULES: [Rule<isize, usize>; 6] = ruleset![
        (0, 0) -> Right(1, 1),
        (0, 1) -> Left(-1, 0),
        (1, 0) -> Right(1, 1),
        (1, 1) -> Left(-1, 1),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Left(0, 1),
    ];

}

#[test]
fn busy_beaver() {
    let initial_state = State(0_isize);
    let input = [0_usize; 10];

    let program = Program::from_iter(RULES.clone());

    let actor = Actor::from_state(initial_state).with_alpha(input);
    let mut rt = actor.execute(program);
    for _ in 0..10 {
        assert!(rt.next().is_some());
    }
}
