/*
    Appellation: tmh <example>
    Created At: 2025.09.03:21:59:56
    Contrib: @FL03
*/
extern crate rstm;

use rstm::prelude::{Program, TMH};

fn main() -> rstm::Result<()> {
    // initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    tracing::info!("Welcome to rstm!");
    // define some input for the machine
    let input = [0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0];
    // initialize the state of the machine
    let initial_state: isize = 0;
    // define the ruleset for the machine
    let program: Program<isize, usize> = rstm::program! {
        #[default_state(initial_state)]
        rules: {
            (0, 0) -> Right(1, 0);
            (0, 1) -> Left(-1, 1);
            (1, 0) -> Right(1, 1);
            (1, 1) -> Right(-1, 0);
            (-1, 0) -> Left(<isize>::MAX, 0);
            (-1, 1) -> Left(0, 1);
        };
    };
    // create a new instance of the machine
    let tm = TMH::new(initial_state, input.to_vec());
    // execute the program
    dbg!(tm).execute(program).run()?;
    Ok(())
}
