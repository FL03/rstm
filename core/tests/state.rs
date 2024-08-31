/*
    Appellation: state <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm_core as rstm;

use rstm::state::{Halt, State};

#[test]
fn state() {
    // create a new instance of state
    let state = State::new(0);
    // validate te functional accessors; get and into_inner
    assert_eq!(state.get_ref(), &0);
    assert_eq!(state.get(), 0);
    // create a new mutable instance of state
    let mut state = State::new(0);
    // replace the inner value with 1
    assert_eq!(state.replace(1), 0);
    // verify the replacement
    assert_eq!(*state.get_ref(), 1);
    // set the inner value to 2
    state.set(2);
    // verify the update
    assert_eq!(*state.get_ref(), 2);
    // reset the state to its default value
    state.reset();
    // verify the reset
    assert_eq!(*state.get_ref(), 0);
    // swap
    state.swap(&mut State::new(10));
    // verify the swap
    assert_eq!(*state.get_ref(), 10);
}

#[test]
fn halting() {
    // create a new instance of state
    let state = Halt::new(0);
    // validate te functional accessors; get and into_inner
    assert_eq!(state.get(), &0);
    assert_eq!(state.into_inner(), 0);
    // create a new mutable instance of state
    let mut state = Halt::new(0);
    // replace the inner value with 1
    assert_eq!(state.replace(1), 0);
    // verify the replacement
    assert_eq!(*state.get(), 1);
    // set the inner value to 2
    state.set(2);
    // verify the update
    assert_eq!(*state.get(), 2);
    // reset the state to its default value
    state.reset();
    // verify the reset
    assert_eq!(*state.get(), 0);
    // swap
    state.swap(&mut Halt::new(10));
    // verify the swap
    assert_eq!(*state.get(), 10);
}

#[test]
fn cast_state() {
    let q = State(0_isize);
    let r = unsafe { q.cast::<usize>() };
    assert_eq!(r.get_ref(), &0_usize);
}

#[test]
fn convert_state() {
    let q = State(0_isize);
    let r = q.into_any();
    assert_eq!(*r.get_ref().downcast_ref::<isize>().unwrap(), 0_isize);
}
