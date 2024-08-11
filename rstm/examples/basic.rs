/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::{rule, Program, State, Tape, TM};

lazy_static::lazy_static! {
    static ref RULES: Program<isize, usize> = Program::from_iter([
        rule![(0, 0) -> Right(0, 0)],
        rule![(0, 1) -> Right(1, 0)],
        rule![(1, 0) -> Right(0, 1)],
        rule![(1, 1) -> Left(1, 0)],
        rule![(-1, 0) -> Right(0, 1)],
        rule![(-1, 1) -> Stay(0, 0)],
    ]);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_target(false).init();

    // initialize the tape data
    let alpha = [0_usize; 10];
    // define the rules for the machine
    let rules = Program::from_iter([
        rule![(0, 0) -> Right(0, 1)],
        rule![(0, 1) -> Right(1, 0)],

        rule![(1, 0) -> Right(-1, 1)],
        rule![(1, 1) -> Left(1, 0)],
        rule![(-1, 0) -> Right(0, 1)],
        rule![(-1, 1) -> Stay(0, 0)],
    ]).with_initial_state(State(0));

    let tape = Tape::from_iter(alpha);
    // create a new instance of the machine
    let tm = TM::new(rules, tape);
    tm.execute()?;
    Ok(())
}
