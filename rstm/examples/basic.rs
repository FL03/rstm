/*
    Appellation: basic <example>
    Created At: 2026.01.11:11:57:40
    Contrib: @FL03
*/
extern crate rstm;

fn main() -> Result<(), rstm::Error> {
    // setup the tracing
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    tracing::debug! { "Welcome to the basic example for the rstmt crate!" }
    // ...
    Ok(())
}
