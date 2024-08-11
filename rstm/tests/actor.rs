/*
    Appellation: actor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use lazy_static::lazy_static;
use rstm::actors::Actor;
use rstm::state::BinState;
use rstm::{rule, Program, State};

use BinState::{Invalid, Valid};

lazy_static! {
    static ref ALPHA: Vec<u8> = vec![1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1];
    static ref RULES: Program<BinState, usize> = Program::from_iter([
        rule![(Invalid, 0) -> Right(Invalid, 0)],
        rule![(Invalid, 1) -> Right(Valid, 0)],
        rule![(Valid, 0) -> Right(Valid, 1)],
        rule![(Valid, 1) -> Left(Valid, 0)],
    ]);
}

#[should_panic]
#[test]
fn test_actor() {
    let input = [0_usize; 10];

    let actor = Actor::new(State(Invalid), input);
    actor.run(RULES.clone());
}
