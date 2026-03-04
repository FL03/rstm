/*
    appellation: fsm <module>
    authors: @FL03
*/
use crate::ast::{FiniteStateMachineAst, RulesBlockAst};
use crate::impls::rule::handle_rule;
use proc_macro2::TokenStream;
use quote::quote;

/// Generates a block expression that builds and returns an initialized
/// `MovingHead` instance from the declared rules and optional default state.
///
/// Types (state `Q` and symbol `A`) are inferred by the compiler from the
/// literal values present in the rule expressions, avoiding the need for
/// explicit type annotations.
pub fn impl_fsm(input: &FiniteStateMachineAst) -> TokenStream {
    let FiniteStateMachineAst {
        default_state,
        rules,
        ..
    } = input;

    let rules_stream = generate_rules(rules);

    // generate the optional `.with_default_state(...)` call
    let with_state = default_state.as_ref().map(|ds| {
        let state = &ds.state;
        quote! { .with_default_state(#state) }
    });

    quote! {
        rstm::MovingHead::tmh(
            rstm::Program::from_iter([
                #(#rules_stream),*
            ])
            #with_state
        )
    }
}

fn generate_rules(rules: &RulesBlockAst) -> Vec<TokenStream> {
    rules.rules.iter().map(handle_rule).collect()
}
