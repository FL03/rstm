/*
    Appellation: tmh <example>
    Created At: 2025.09.03:21:59:56
    Contrib: @FL03
*/
extern crate rstm;

use rstm::actors::TMH;

fn main() -> rstm::Result<()> {
    // initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    // define some input for the machine
    let input = [0, 0, 0, 0, 1, 0, 1, 1, 0, 1];
    // initialize the state of the machine
    let initial_state: isize = 0;
    // define the Program for the machine
    let program = rstm::program! {
        default_state: 0,
        rules: {
            (0, 0) -> Right(1, 0usize),
            (0, 1) -> Right(-1, 1),
            (1, 0) -> Right(1, 1),
            (1, 1) -> Right(0, 0),
            (-1, 0) -> Left(<isize>::MAX, 0),
            (-1, 1) -> Left(-1, 0),
        };
    };
    // export the program to a JSON file
    program.export_json("rstm/examples/example.program.json")?;
    // create a new instance of the machine
    let mut tm = TMH::from_state(initial_state).with_tape(input);
    // execute and run the program
    tm.execute(program).run()?;
    Ok(())
}
