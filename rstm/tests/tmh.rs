/*
    Appellation: tmh <test>
    Created At: 2025.09.03:23:13:14
    Contrib: @FL03
*/
extern crate rstm;

use rstm::prelude::{Program, TMH};

/// Define the default state for the machine(s)
const DEFAULT_STATE: isize = 0;

lazy_static::lazy_static! {
    /// A common program to be executed by the machine(s)
    static ref PROGRAM: Program<isize, usize> = rstm::program! {
        #[default_state(DEFAULT_STATE)]
        rules: {
            (0, 0) -> Right(1, 0);
            (0, 1) -> Left(-1, 1);
            (1, 0) -> Right(1, 1);
            (1, 1) -> Right(0, 0);
            (-1, 0) -> Left(<isize>::MAX, 0);
            (-1, 1) -> Left(-1, 0);
        };
    };
}


#[test]
fn test_tmh() -> rstm::Result<()> {
    // define some input for the machine
    let input = vec![0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0];
    // create a new instance of the machine
    let mut tm = TMH::new(DEFAULT_STATE, input);
    // execute and run the program
    assert!(tm.execute(PROGRAM.clone()).run().is_ok());
    Ok(())
}

#[test]
#[ignore = "test not implemented yet"]
fn busy_beaver() -> rstm::Result<()> {
    Ok(())
}