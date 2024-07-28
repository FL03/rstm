/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::prelude::{State, StdTape, TM};
use rstm::rule;
use rstm::state::binary::BinaryStates::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    println!("{}", -1_isize as u8);
    let tape: StdTape<u8> =
        StdTape::from_iter([0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1]);
    let initial_state = State(Invalid);

    let rules = vec![
        rule![(Invalid, 0) -> Left(Invalid, 0)],
        rule![(Invalid, 1) -> Right(Valid, 0)],
        rule![(Valid, 0) -> Right(Valid, 1)],
        rule![(Valid, 1) -> Left(Valid, 0)],
    ];

    let tm = TM::new(initial_state, rules, tape);
    tm.run()?;
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Three {
    #[default]
    A = 0,
    B = 1,
    C = 2,
}

impl rstm::Alphabet for Three {
    type Elem = char;

    fn symbols(&self) -> Vec<char> {
        vec!['A', 'B', 'C']
    }
}

pub enum Two<T> {
    X(T),
    Y(T),
}
