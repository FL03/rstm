/*
    Appellation: ast <module>
    Created At: 2026.03.04:15:02:12
    Contrib: @FL03
*/
//! abstract syntax trees (ast) for the procedural macros
#[doc(inline)]
#[allow(unused_imports)]
pub use self::{fsm_ast::*, rule_ast::*};

pub mod fsm_ast;
pub mod rule_ast;
