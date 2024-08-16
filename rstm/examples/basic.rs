/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::{ruleset, Program, State, Tape, TM};

lazy_static::lazy_static! {
    static ref RULES: Program<isize, usize> = Program::from_iter(ruleset![
        (0, 0) -> Right(0, 1),
        (0, 1) -> Right(1, 0),
        (1, 0) -> Right(-1, 1),
        (1, 1) -> Left(1, 0),
        (-1, 0) -> Right(0, 1),
        (-1, 1) -> Stay(0, 0),
    ]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_target(false).init();

    // initialize the tape data
    let alpha = [0_usize; 10];
    // define the rules for the machine
    let rules = ruleset![
        (0, 0) -> Right(0, 1),
        (0, 1) -> Right(1, 0),
        (1, 0) -> Right(-1, 1),
        (1, 1) -> Left(1, 0),
        (-1, 0) -> Right(0, 1),
        (-1, 1) -> Stay(0, 0),
    ];
    // create a new program with the rules
    let program = Program::new().initial_state(State(0)).rules(rules).build();
    // create a new tape with the data
    let tape = Tape::from_iter(alpha);
    // create a new instance of the machine
    let tm = TM::new(program, tape);
    tm.execute()?;
    Ok(())
}
