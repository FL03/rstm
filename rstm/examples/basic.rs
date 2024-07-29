/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::state::BinState::{Invalid, Valid};
use rstm::{rule, Program, State, StdTape, TM};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_target(false).init();

    // initialize the tape data
    let alpha: Vec<u8> = vec![1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1];
    // define the rules for the machine
    let rules = vec![
        rule![(Invalid, 0) -> Right(Invalid, 0)],
        rule![(Invalid, 1) -> Right(Valid, 0)],
        rule![(Valid, 0) -> Right(Valid, 1)],
        rule![(Valid, 1) -> Left(Valid, 0)],
    ];

    let tape = StdTape::from_iter(alpha);
    let program = Program::from_state(State(Invalid)).with_instructions(rules);
    // create a new instance of the machine
    let tm = TM::new(program, tape);
    tm.execute()?;
    Ok(())
}
