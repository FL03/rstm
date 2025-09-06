/*
    Appellation: tape <test>
    Created At: 2025.09.05:22:13:26
    Contrib: @FL03
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
