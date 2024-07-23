/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::rule;
use rstm::state::binary::BinaryStates::*;
use rstm::prelude::{Direction, Fsm, Instruction, State, Tape};

use Direction::Right;

lazy_static::lazy_static! {
    static ref RULESET: Vec<Instruction<BinaryStates, char>> = vec![
        rule![(State(Invalid), '1') -> Right(State(Valid), '0',)],
        rule![(State(Invalid), '1') -> Right(State(Valid), '0',)],
        rule![(State(Invalid), '0') -> Right(State(Valid), '1',)],
        rule![(State(Valid), '1') -> Right(State(Invalid), '0',)],
    ];
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    let tape = Tape::from_str("1011");
    let initial_state = State(Invalid);

    let rules = vec![
        
    ];

    let mut tm = Fsm::new(initial_state, RULESET.clone(), tape);
    tm.run();
    Ok(())
}
