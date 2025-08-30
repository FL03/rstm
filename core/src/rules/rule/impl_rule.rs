/*
    appellation: impl_rule <module>
    authors: @FL03
*/
use super::{Rule, RuleBuilder};
use crate::rules::{Head, Tail};
use crate::state::{RawState, State};

impl<Q, A> Rule<Q, A>
where
    Q: RawState,
{
    pub fn new() -> RuleBuilder<Q, A> {
        RuleBuilder::new()
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
    pub const fn tail(&self) -> &Tail<Q, A> {
        &self.tail
    }
    /// returns a mutable reference to the [Tail] of the [Instruction]
    pub const fn tail_mut(&mut self) -> &mut Tail<Q, A> {
        &mut self.tail
    }
    /// returns an instance of the [Head] whose elements are immutable references
    pub fn head_view(&self) -> Head<&'_ Q, &'_ A> {
        self.head().view()
    }
    /// returns an instance of the [Tail] whose elements are immutable references
    pub const fn tail_view(&self) -> Tail<&'_ Q, &'_ A> {
        self.tail().view()
    }
    /// returns the direction of the shift
    pub const fn direction(&self) -> crate::Direction {
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
    pub fn symbol_mut(&mut self) -> &mut A {
        self.head_mut().symbol_mut()
    }
    /// returns the next [State] of the system
    pub const fn next_state(&self) -> &State<Q> {
        self.tail().state()
    }
    /// returns a mutable reference to the next [State] of the system
    pub const fn next_state_mut(&mut self) -> &mut State<Q> {
        self.tail_mut().state_mut()
    }
    /// returns the symbol which will be written by the [Head]
    pub const fn next_symbol(&self) -> &A {
        self.tail().symbol()
    }
    /// returns a mutable reference to the next symbol
    pub fn next_symbol_mut(&mut self) -> &mut A {
        self.tail_mut().symbol_mut()
    }
    /// updates the current [Direction] and returns a mutable reference to the [Rule]
    pub fn set_direction(&mut self, direction: crate::Direction) -> &mut Self {
        self.tail_mut().set_direction(direction);
        self
    }
    /// update the current symbol and return a mutable reference to the [Rule]
    pub fn set_symbol(&mut self, symbol: A) -> &mut Self {
        self.head_mut().set_symbol(symbol);
        self
    }
    /// updates the current [State] and returns a mutable reference to the [Rule]
    pub fn set_state(&mut self, state: State<Q>) -> &mut Self {
        self.head_mut().set_state(state);
        self
    }
    /// updates the current [State] and returns a mutable reference to the [Rule]
    pub fn set_next_state(&mut self, state: State<Q>) -> &mut Self {
        self.tail_mut().set_state(state);
        self
    }
    /// updates the next symbol and returns a mutable reference to the [Rule]
    pub fn set_next_symbol(&mut self, symbol: A) -> &mut Self {
        self.tail_mut().set_symbol(symbol);
        self
    }
    /// updates the current [State] and symbol and returns a mutable reference to the [Rule]
    pub fn set_head(&mut self, state: State<Q>, symbol: A) -> &mut Self {
        self.head_mut().set_state(state);
        self.head_mut().set_symbol(symbol);
        self
    }
    /// updates the current [State] and symbol and returns a mutable reference to the [Rule]
    pub fn set_tail(&mut self, state: State<Q>, symbol: A) -> &mut Self {
        self.tail_mut().set_state(state);
        self.tail_mut().set_symbol(symbol);
        self
    }
    /// returns the next [Head] of the system
    pub fn next_head(&self) -> Head<&'_ Q, &'_ A> {
        self.tail().as_head()
    }
    /// consumes the current object and returns the next [Head] of the system
    pub fn into_next_head(self) -> Head<Q, A> {
        self.tail.into_head()
    }
    /// returns the value which for which the current object will be replaced with
    pub const fn write_symbol(&self) -> &A {
        self.tail().symbol()
    }
    /// consumes the current object and returns a 2-tuple consisting of the [Head] and [Tail]
    pub fn into_tuple(self) -> (Head<Q, A>, Tail<Q, A>) {
        (self.head, self.tail)
    }
    /// returns a new instance of the [`Rule`] with cloned elements
    pub fn cloned(&self) -> Rule<Q, A>
    where
        Q: Clone,
        A: Clone,
    {
        Rule {
            head: self.head.clone(),
            tail: self.tail.clone(),
        }
    }
    /// returns a new instance of the [`Rule`] with copied elements
    pub fn copied(&self) -> Rule<Q, A>
    where
        Q: Clone,
        A: Clone,
    {
        Rule {
            head: self.head.clone(),
            tail: self.tail.clone(),
        }
    }
}
