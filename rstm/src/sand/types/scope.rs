/*
    Appellation: agent <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::state::State;

pub struct Observer<Q = String> {
    pub position: usize,
    pub state: State<Q>,
}
