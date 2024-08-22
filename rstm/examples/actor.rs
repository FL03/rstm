/*
    Appellation: actor <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::{ruleset, Actor, Program, State};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    _tracing();
    // initialize the tape data
    let alpha: Vec<i8> = vec![0, 1, 1, 0, 1, 0, 1, 0];
    // initialize the state of the machine
    let initial_state = State(0);
    // define the ruleset for the machine
    let rules = ruleset![
        (0, 0) -> Right(1, 0),
        (0, 1) -> Right(-1, 1),
        (1, 0) -> Right(0, 1),
        (1, 1) -> Right(-1, 0),
        (-1, 0) -> Left(0, 0),
        (-1, 1) -> Left(1, 1),
    ];

    let program = Program::new()
        .initial_state(initial_state)
        .rules(rules)
        .build();

    // create a new instance of the machine
    let tm = dbg!(Actor::new(initial_state).with_tape(alpha));
    let out = tm.execute(program).run()?;
    println!("Output: {out:?}");
    Ok(())
}

fn _tracing() {
    let timer = tracing_subscriber::fmt::time::uptime();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_timer(timer)
        .init();
    tracing::info!("Welcome to rstm!");
}
