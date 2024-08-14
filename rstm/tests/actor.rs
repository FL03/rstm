/*
    Appellation: actor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use lazy_static::lazy_static;
use rstm::actors::Actor;
use rstm::{rule, Program, State};

lazy_static! {
    static ref ALPHA: Vec<u8> = vec![1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1];
    static ref RULES: Program<isize, usize> = Program::from_iter([
        rule![(0, 0) -> Right(0, 0)],
        rule![(0, 1) -> Right(1, 0)],
        rule![(1, 0) -> Right(0, 1)],
        rule![(1, 1) -> Left(1, 0)],
        rule![(-1, 0) -> Right(0, 1)],
        rule![(-1, 1) -> Stay(0, 0)],
    ]);
}

#[should_panic]
#[test]
fn test_actor() {
    let input = [0_usize; 10];

    let actor = Actor::new().alpha(input).state(State(0)).build();
    actor.exec(RULES.clone());
}
