/*
    Appellation: tape <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstm_tape::StdTape;

#[test]
fn std_tape() {
    // initialize a new tape
    let mut tape = StdTape::<u8>::new();
    // write some symbols
    tape.write(0);
    tape.write(1);
    //
    assert_eq!(tape.read().ok(), Some(&1));
}
