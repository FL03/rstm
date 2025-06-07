/*
    Appellation: tape <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm_core as rstm;

use rstm::mem::tape::StdTape;

#[test]
fn stdtape() {
    let mut tape = StdTape::<u8>::new();
    tape.write(0);
    tape.write(1);

    assert_eq!(tape.read().ok(), Some(&1));
}
