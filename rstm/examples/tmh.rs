/*
    Appellation: tmh <example>
    Created At: 2025.09.03:21:59:56
    Contrib: @FL03
*/
extern crate rstm;

use rstm::{Head, program};

fn main() -> rstm::Result<()> {
    // initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    // define some input for the machine
    let input = vec![0, 0, 0, 0, 1, 0, 1, 1, 0, 1];
    // initialize the state of the machine
    let initial_state: isize = 0;
    // define the Program for the machine
    let program = program! {
        #[default_state(initial_state)]
        rules: {
            (0, 0) -> Right(1, 0usize),
            (0, 1) -> Left(-1, 1),
            (1, 0) -> Right(1, 1),
            (1, 1) -> Right(0, 0),
            (-1, 0) -> Left(<isize>::MAX, 0),
            (-1, 1) -> Left(-1, 0),
        };
    };
    // export the program to a JSON file
    program.export_json("rstm/examples/example.program.json")?;
    // create a new instance of the machine
    let mut tm = Head::new(initial_state, 0usize).load(program);
    // load the input into the machine tape
    tm.extend_tape(input);
    // execute the program
    tm.run()
}
