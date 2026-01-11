/*
    Appellation: head2tails <module>
    Created At: 2025.09.05:17:54:35
    Contrib: @FL03
*/
extern crate rstm_core as rstm;
use rstm::{Direction, Head, Tail};

#[test]
fn test_head() {
    let mut head = Head::new(100_usize, 'a');
    assert_eq!(head.state(), &100_usize);
    assert_eq!(head.symbol(), &'a');
    // update the head
    head.set_state(200_usize);
    head.set_symbol('b');
    assert_eq!(head.state(), &200_usize);
    assert_eq!(head.symbol(), &'b');
    let head_ref = head.view();
    assert_eq!(head_ref.state(), &&200_usize);
}

#[test]
fn test_head_tail_add() {
    let head = Head::new(100_usize, 'a');
    let tail = Tail::new(Direction::Right, 300_usize, 'b');
    let rule = head + tail;
    assert_eq!(rule.head.state(), &100_usize);
    assert_eq!(rule.head.symbol(), &'a');
    assert_eq!(rule.tail.state(), &300_usize);
    assert_eq!(rule.tail.symbol(), &'b');
}
