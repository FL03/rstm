/*
    Appellation: actor <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm;

use rstm::actors::TMH;
use rstm::rules::Program;

fn main() -> rstm::Result<()> {
    // initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    tracing::info!("Welcome to rstm!");
    // initialize the tape data
    let alpha = [0_usize; 10];
    // initialize the state of the machine
    let initial_state: isize = 0;
    // define the ruleset for the machine
    let program: Program<isize, usize> = rstm::program! {
        #[default_state(initial_state)]
        rules: {
            (0, 0) -> Right(1, 0);
            (0, 1) -> Left(-1, 1);
            (1, 0) -> Right(0, 1);
            (1, 1) -> Right(-1, 0);
            (-1, 0) -> Left(0, 0);
            (-1, 1) -> Left(1, 1);
        };
    };
    // create a new instance of the machine
    let mut tm = TMH::from_state(initial_state);
    tm.extend_tape(alpha);
    // execute the program
    dbg!(tm).execute(program).run()?;
    tracing::info!("Execution successfull! Terminating the program...");
    Ok(())
}
