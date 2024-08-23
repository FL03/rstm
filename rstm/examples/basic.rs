/*
    Appellation: default <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::{ruleset, Program, State, StdTape, Turm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    _tracing("debug");

    // initialize the tape data
    let alpha = [0_usize; 10];
    // define the rules for the machine
    let rules = ruleset![
        (0, 0) -> Right(1, 0),
        (0, 1) -> Right(-1, 1),
        (1, 0) -> Right(0, 1),
        (1, 1) -> Right(-1, 0),
        (-1, 0) -> Left(0, 0),
        (-1, 1) -> Left(1, 1),
    ];
    // create a new program with the rules
    let program = Program::new().initial_state(State(0)).rules(rules).build();
    // create a new tape with the data
    let tape = StdTape::from_iter(alpha);
    // create a new instance of the machine
    let tm = Turm::new(program, tape);
    tm.execute()?;
    Ok(())
}

fn _tracing(level: &str) {
    let level = match level {
        "debug" => tracing::Level::DEBUG,
        "error" => tracing::Level::ERROR,
        "trace" => tracing::Level::TRACE,
        "warn" => tracing::Level::WARN,
        _ => tracing::Level::INFO,
    };
    let timer = tracing_subscriber::fmt::time::uptime();
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_timer(timer)
        .init();
    tracing::info!("Welcome to rstm!");
}
