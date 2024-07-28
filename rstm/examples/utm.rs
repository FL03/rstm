/*
    Appellation: wolfram <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::{
    rule,
    state::{self, State},
    StdTape, TM,
};

use state::BinaryStates::*;

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

pub mod wolfram {

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, strum::EnumIter)]
    #[repr(u8)]
    pub enum Three {
        #[default]
        A = 0,
        B = 1,
        C = 2,
    }

    impl Three {
        pub fn as_u8(&self) -> u8 {
            *self as u8
        }

        pub fn iter() -> impl Iterator<Item = Three> {
            use Three::*;
            [A, B, C].into_iter()
        }

        pub fn iter_value() -> impl Iterator<Item = u8> {
            [0, 1, 2].into_iter()
        }
    }

    impl rstm::Alphabet for Three {
        type Sym = Self;

        fn symbols_to_vec(&self) -> Vec<Self::Sym> {
            use Three::*;
            vec![A, B, C]
        }
    }

    impl IntoIterator for Three {
        type Item = Self;
        type IntoIter = std::vec::IntoIter<Self>;

        fn into_iter(self) -> Self::IntoIter {
            vec![Three::A, Three::B, Three::C].into_iter()
        }
    }

    pub enum Two<T> {
        A(T),
        B(T),
    }
}
