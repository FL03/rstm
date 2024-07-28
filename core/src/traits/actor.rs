/*
    Appellation: fsm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Automata {
    type Rule;
    type State;
    type Symbol;

    fn current_state(&self) -> Self::State;
}
