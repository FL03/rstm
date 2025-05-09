/*
    Appellation: state <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstm_core as rstm;

use rstm::state::{Halt, State};

#[test]
fn state() {
    // create a new instance of state
    let state = State::from_value(0);
    // validate the functional accessors; get and into_inner
    assert_eq!(state.get(), &0);
    assert_eq!(state.into_inner(), 0);
    // create a new mutable instance of state
    let mut state = State::from_value(0);
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
    state.swap(&mut State::from_value(10));
    // verify the swap
    assert_eq!(*state.get(), 10);
}

#[test]
fn halting() {
    // create a new instance of state
    let state = Halt::new(0);
    // validate the functional accessors; get and into_inner
    assert_eq!(state.get_ref(), &0);
    assert_eq!(state.get(), 0);
    // create a new mutable instance of state
    let mut state = Halt::new(0);
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
    state.swap(&mut Halt::new(10));
    // verify the swap
    assert_eq!(*state.get_ref(), 10);
}

#[test]
fn cast_state() {
    // create a new instance of state with a value of 0_isize
    let q = State(0_isize);
    // cast the state to a usize
    let r = unsafe { q.cast::<usize>() };
    // verify the casted state has an inner value of 0
    assert_eq!(r, 0_usize);
}

#[test]
fn convert_state() {
    // create a new instance of state with a value of 0
    let q = State(0_isize);
    // convert the state into an Any
    let r = q.into_any();
    // verify the converted state has an inner value of 0
    assert_eq!(*r.get().downcast_ref::<isize>().unwrap(), 0_isize);
}

#[test]
fn numstate() {
    use num::{One, Zero};
    // create a new instance of state with a value of 1
    let one = State::<isize>::one();
    // create a new instance of state with a value of 0_f64
    let zero = State::<f32>::zero();
    // verify the state has an inner value of 1
    assert_eq!(one, 1);
    // verify the state has an inner value of 0
    assert_eq!(zero, 0.0);
}
