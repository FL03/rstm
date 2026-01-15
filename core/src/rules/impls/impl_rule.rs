/*
    appellation: impl_rule <module>
    authors: @FL03
*/
use crate::rules::{Rule, RuleBuilder};

use crate::{Direction, Head, Tail};
use rstm_state::{RawState, State};

impl<Q, A, R, B> Rule<Q, A, R, B>
where
    Q: RawState,
    R: RawState,
{
    /// returns a new instance of the [`Rule`] from the given head and tail
    pub const fn new(head: Head<Q, A>, tail: Tail<R, B>) -> Self {
        Self { head, tail }
    }
    /// returns a new instance of a [`RuleBuilder`] for constructing a new [`Rule`]
    pub const fn init() -> RuleBuilder<Q, A, R, B> {
        RuleBuilder::new()
    }
    /// initialize a new instance of the [`Rule`] from its consituent parts
    pub const fn from_parts(
        state: Q,
        symbol: A,
        direction: Direction,
        next_state: R,
        write_symbol: B,
    ) -> Self {
        let head = Head::new(state, symbol);
        let tail = Tail::new(direction, next_state, write_symbol);
        Self { head, tail }
    }
    /// consumes the current object to create another with the given head
    pub fn with_head(self, head: Head<Q, A>) -> Self {
        Self { head, ..self }
    }
    /// consumes the current object to create another with the given tail
    pub fn with_tail(self, tail: Tail<R, B>) -> Self {
        Self { tail, ..self }
    }
    /// returns an immutable reference to the [Head]
    pub const fn head(&self) -> &Head<Q, A> {
        &self.head
    }
    /// returns a mutable reference to the [Head]
    pub const fn head_mut(&mut self) -> &mut Head<Q, A> {
        &mut self.head
    }
    /// returns an immutable reference to the [Tail] of the [Instruction]
    pub const fn tail(&self) -> &Tail<R, B> {
        &self.tail
    }
    /// returns a mutable reference to the [Tail] of the [Instruction]
    pub const fn tail_mut(&mut self) -> &mut Tail<R, B> {
        &mut self.tail
    }
    /// returns an instance of the [Head] whose elements are immutable references
    pub const fn head_view(&self) -> Head<&'_ Q, &'_ A> {
        self.head().view()
    }
    /// returns an instance of the [Tail] whose elements are immutable references
    pub const fn tail_view(&self) -> Tail<&'_ R, &'_ B> {
        self.tail().view()
    }
    /// returns the direction of the shift
    pub const fn direction(&self) -> Direction {
        self.tail().direction()
    }
    /// returns the current [State] of the system
    pub const fn state(&self) -> &State<Q> {
        self.head().state()
    }
    /// returns a mutable reference to the current [State] of the system
    pub const fn state_mut(&mut self) -> &mut State<Q> {
        self.head_mut().state_mut()
    }
    /// returns the symbol of the [Head]
    pub const fn symbol(&self) -> &A {
        self.head().symbol()
    }
    /// returns a mutable reference to the symbol of the [`Head`]
    pub const fn symbol_mut(&mut self) -> &mut A {
        self.head_mut().symbol_mut()
    }
    /// returns the next [State] of the system
    pub const fn next_state(&self) -> &State<R> {
        self.tail().state()
    }
    /// returns a mutable reference to the next [State] of the system
    pub const fn next_state_mut(&mut self) -> &mut State<R> {
        self.tail_mut().state_mut()
    }
    /// returns the symbol which will be written by the [Head]
    pub const fn next_symbol(&self) -> &B {
        self.tail().symbol()
    }
    /// returns a mutable reference to the next symbol
    pub const fn next_symbol_mut(&mut self) -> &mut B {
        self.tail_mut().symbol_mut()
    }
    /// returns the value which for which the current object will be replaced with
    pub const fn write_symbol(&self) -> &B {
        self.tail().symbol()
    }
    /// returns a mutable reference to the next symbol
    pub const fn write_symbol_mut(&mut self) -> &mut B {
        self.tail_mut().symbol_mut()
    }
    /// updates the current [Direction] and returns a mutable reference to the [Rule]
    pub fn set_direction(&mut self, direction: Direction) {
        self.tail_mut().set_direction(direction);
    }
    /// update the current symbol and return a mutable reference to the [Rule]
    pub fn set_symbol(&mut self, symbol: A) {
        self.head_mut().set_symbol(symbol);
    }
    /// updates the current [State] and returns a mutable reference to the [Rule]
    pub fn set_state(&mut self, state: Q) {
        self.head_mut().set_state(state);
    }
    /// updates the current [State] and returns a mutable reference to the [Rule]
    pub fn set_next_state(&mut self, state: R) {
        self.tail_mut().set_state(state);
    }
    /// updates the next symbol and returns a mutable reference to the [Rule]
    pub fn set_next_symbol(&mut self, symbol: B) {
        self.tail_mut().set_symbol(symbol);
    }
    /// updates the current [State] and symbol and returns a mutable reference to the [Rule]
    pub fn set_head(&mut self, state: Q, symbol: A) {
        self.head_mut().set_state(state);
        self.head_mut().set_symbol(symbol);
    }
    /// updates the current [State] and symbol and returns a mutable reference to the [Rule]
    pub fn set_tail(&mut self, state: R, symbol: B) {
        self.tail_mut().set_state(state);
        self.tail_mut().set_symbol(symbol);
    }
    /// consumes the current object and returns a 2-tuple consisting of the [Head] and [Tail]
    pub fn into_tuple(self) -> (Head<Q, A>, Tail<R, B>) {
        (self.head, self.tail)
    }
    /// returns the next [Head] of the system
    pub const fn next_head(&self) -> Head<&'_ R, &'_ B> {
        self.tail().as_head()
    }
    /// consumes the current object and returns the next [Head] of the system
    pub fn into_next_head(self) -> Head<R, B> {
        self.tail.into_head()
    }
}
