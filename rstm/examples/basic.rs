/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::prelude::{Direction, Instruction, State, Tape, TM};
use rstm::rule;
use rstm::state::binary::BinaryStates::*;

// use Direction::Right;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    let tape = Tape::from_str("1011");
    let initial_state = State(Invalid);

    let rules = vec![
        rule![(State(Invalid), '0') -> Right(State(Invalid), '1',)],
        rule![(State(Invalid), '1') -> Right(State(Valid), '0',)],
        rule![(State(Valid), '0') -> Right(State(Valid), '1',)],
        rule![(State(Valid), '1') -> Right(State(Valid), '0',)],
    ];

    let mut tm = TM::new(initial_state, rules, tape);
    tm.run();
    Ok(())
}
