/*
    Appellation: wolfram <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::prelude::{State, StdTape, TM};
use rstm::rule;
use rstm::state::binary::BinaryStates::*;

// use Direction::Right;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_level(true).init();
    tracing::info!("Example: Wolfram [2, 3] UTM");

    let tape = StdTape::<u8>::from_iter([0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1]);
    let initial_state = State(Invalid);

    let rules = vec![
        rule![(Invalid, 0) -> Right(Invalid, 0)],
        rule![(Invalid, 1) -> Right(Valid, 0)],
        rule![(Valid, 0) -> Right(Valid, 1)],
        rule![(Valid, 1) -> Right(Valid, 0)],
    ];

    let tm = TM::new(initial_state, rules, tape);
    tm.run()?;
    Ok(())
}

mod wolfram {

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
        A(T),
        B(T),
    }
}
