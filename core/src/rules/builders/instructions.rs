/*
    Appellation: instructions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Default)]
pub struct RulesetBuilder<Q, S> {
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) rules: Vec<Rule<Q, S>>,
}