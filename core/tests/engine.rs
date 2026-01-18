/*
    Appellation: engine <tests>
    Created At: 2025.09.03:21:59:56
    Contrib: @FL03
*/
use rstm_core::{HeadEngine, program};

#[test]
fn test_head_engine() {
    let input = [0i8, 0, 0, 0, 1, 0, 1, 1, 0, 1];
    // initialize the state of the machine
    let initial_state: isize = 0;
    // define the Program for the machine
    let program = program! {
        #[default_state(initial_state)]
        rules: {
            (0, 0) -> Right(1, 0),
            (0, 1) -> Left(-1, 1),
            (1, 0) -> Right(1, 1),
            (1, 1) -> Right(0, 0),
            (-1, 0) -> Left(<isize>::MAX, 0),
            (-1, 1) -> Left(-1, 0),
        };
    };
    // create a new instance of the machine
    let mut tm = HeadEngine::tmh(program);
    // load the input into the machine tape
    tm.extend_tape(input);
    // execute the program
    let res = tm.run();
    assert! { res.is_ok() && tm.is_halted() }
    assert_eq! { tm.cycles(), 9 }
}
