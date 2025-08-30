/*
    Appellation: actor <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstm::{Actor, InstructionSet, State};

#[test]
#[ignore = "the halting state needs to be fixed/enabled"]
fn busy_beaver() {
    let initial_state = State(0_isize);
    let input = [0_usize; 10];

    let program: InstructionSet<isize, usize> = rstm::ruleset! {
        #[default_state(*initial_state)] // optional
        (0, 0) -> Right(1, 1),
        (0, 1) -> Left(-1, 0),
        (1, 0) -> Right(1, 1),
        (1, 1) -> Left(-1, 1),
        (-1, 0) -> Right(0, 0),
        (-1, 1) -> Left(0, 1),
    };

    let actor = Actor::from_state(initial_state).with_tape(input);
    let mut rt = actor.execute(program);
    for _ in 0..10 {
        assert!(rt.next().is_some());
    }
}
