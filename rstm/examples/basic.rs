/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::prelude::{State, StdTape, TM};
use rstm::rule;
use rstm::state::binary::BinaryStates::*;

// use Direction::Right;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    let tape = StdTape::from_str("10111000101001101011010010");
    let initial_state = State(Invalid);

    let rules = vec![
        rule![(State(Invalid), '0') -> Right(State(Invalid), '1',)],
        rule![(State(Invalid), '1') -> Right(State(Valid), '0',)],
        rule![(State(Valid), '0') -> Right(State(Valid), '1',)],
        rule![(State(Valid), '1') -> Right(State(Valid), '0',)],
    ];

    let tm = TM::new(initial_state, rules, tape);
    tm.run()?;
    Ok(())
}
