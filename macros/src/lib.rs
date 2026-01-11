/*
    Appellation: rstm-macros <library>
    Created At: 2026.01.11:11:33:56
    Contrib: @FL03
*/
//! procedural macros for interacting with various wrappers
extern crate proc_macro;

mod ast {
    pub use self::{fsm_ast::*, rule_ast::*};

    mod fsm_ast;
    mod rule_ast;
}

mod impls {
    pub use self::{fsm::impl_wrapper_binary_ops, rule::impl_rule};

    pub mod fsm;
    pub mod rule;
}

pub(crate) mod keywords {
    syn::custom_keyword! { direction }
    syn::custom_keyword! { state }
    syn::custom_keyword! { symbol }

    syn::custom_keyword! { head }
    syn::custom_keyword! { tail }

    syn::custom_keyword! { rule }
}

use crate::ast::{FiniteStateMachineAst, RuleAst};
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// The [`ruler!`] generates a finite state machine implementation
///
/// ```no_run
/// ruler![(0, 'a') -> Right(1i8, 'b')];
/// ```
#[proc_macro]
pub fn ruler(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as RuleAst);
    let output = impls::impl_rule(&ast);
    output.into()
}
/// The [`fsm!`] generates a finite state machine implementation
///
/// ```rust
/// 
/// ```
#[proc_macro]
pub fn fsm(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as FiniteStateMachineAst);
    let output = impls::impl_wrapper_binary_ops(ast);
    output.into()
}
