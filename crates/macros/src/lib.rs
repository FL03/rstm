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

use self::ast::FiniteStateMachineAst;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// The [`fsm!`] generates a finite state machine implementation
///
/// ## Syntax
///
/// ```ignore
/// fsm! {
///     default_state: 0; // optional
///     rules: {
///      (state, symbol) -> Direction(next_state, next_symbol),
///         ...
///     };
/// }
/// ```
#[proc_macro]
pub fn fsm(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as FiniteStateMachineAst);
    let output = impls::impl_fsm(&ast);
    output.into()
}
