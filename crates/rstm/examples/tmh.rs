/*
    Appellation: tmh <example>
    Created At: 2025.09.03:21:59:56
    Contrib: @FL03
*/
extern crate rstm;

fn main() -> rstm::Result<()> {
    // initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    // define some input for the machine
    let input = [0, 0, 0, 0, 1, 0, 1, 1, 0, 1];
    // initialize the machine using the `fsm!` macro
    let mut fsm = rstm::tmh! {
        default_state: 0isize;
        rules: {
            (0, 0) -> Right(1, 0),
            (0, 1) -> Left(-1, 1),
            (1, 0) -> Right(1, 1),
            (1, 1) -> Right(0, 0),
            (-1, 0) -> Left(<isize>::MAX, 0),
            (-1, 1) -> Left(-1, 0),
        }
    };
    // optionally, export the program to a JSON file
    fsm.program()
        .expect("Failed to get program")
        .export_json("./crates/rstm/examples/example.program.json")?;
    // create a new instance of the machine
    // let mut tm = MovingHead::tmh(program);
    // load the input into the machine tape
    fsm.extend_tape(input);
    // execute the program
    fsm.run()
}
