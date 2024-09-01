/*
    Appellation: actor <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::{ruleset, Actor, Program, State};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    _tracing("debug");
    // initialize the tape data
    let alpha = vec![0u8; 10];
    // initialize the state of the machine
    let initial_state = State(0isize);
    // define the ruleset for the machine
    let rules = ruleset![
        (0, 0) -> Right(1, 0),
        (0, 1) -> Right(-1, 1),
        (1, 0) -> Right(0, 1),
        (1, 1) -> Right(-1, 0),
        (-1, 0) -> Left(0, 0),
        (-1, 1) -> Left(1, 1),
    ];
    // create a new program from the ruleset
    let program = Program::from_iter(rules);
    // create a new instance of the machine
    let tm = dbg!(Actor::new(alpha, initial_state, 0));
    tm.execute(program).run()?;
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
