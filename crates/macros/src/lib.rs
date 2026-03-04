/*
    Appellation: rstm-macros <library>
    Created At: 2026.01.11:11:33:56
    Contrib: @FL03
*/
//! procedural macros for interacting with various wrappers
extern crate proc_macro;

pub(crate) mod ast;
pub(crate) mod impls;
pub(crate) mod keywords;

use self::ast::{FiniteStateMachineAst, RuleAst};
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
pub fn tmh(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as FiniteStateMachineAst);
    let output = impls::impl_fsm(&ast);
    output.into()
}
