/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::{
    rule,
    state::{self, State},
    StdTape, TM,
};

use state::BinState::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_target(false).init();
    println!("{}", -1_isize as u8);
    let tape_data: Vec<u8> = vec![0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1];

    let rules = vec![
        rule![(Invalid, 0) -> Right(Invalid, 0)],
        rule![(Invalid, 1) -> Right(Valid, 0)],
        rule![(Valid, 0) -> Right(Valid, 1)],
        rule![(Valid, 1) -> Left(Valid, 0)],
    ];

    let tape = StdTape::from_iter(tape_data);
    let tm = TM::new(State(Invalid), rules, tape);
    tm.run()?;
    Ok(())
}
