/*
    appellation: fsm <module>
    authors: @FL03
*/
use crate::ast::{FiniteStateMachineAst, HeadAst, RuleAst, RuleBlockAst, TailAst};
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
    // convert the rules into token streams
    let rules_stream = handle_rule_block(rules);
    // generate the optional `.with_default_state(...)` call
    let with_state = default_state.as_ref().map(|ds| {
        let state = &ds.state;
        quote! { .with_default_state(#state) }
    });

    quote! {
        rstm::MovingHead::tmh(
            rstm::Program::from_iter([#(#rules_stream),*]) #with_state
        )
    }
}

fn handle_rule_block(RuleBlockAst { rules, .. }: &RuleBlockAst) -> Vec<TokenStream> {
    rules.into_iter().map(handle_rule).collect()
}

fn handle_rule(
    RuleAst {
        head: HeadAst { state, symbol, .. },
        tail:
            TailAst {
                direction,
                next_state,
                next_symbol,
                ..
            },
        ..
    }: &RuleAst,
) -> TokenStream {
    // create a rule
    quote! {
        rstm::Rule {
            head: rstm::Head { state: #state, symbol: #symbol },
            tail: rstm::Tail {
                direction: rstm::Direction::#direction,
                next_state: #next_state,
                write_symbol: #next_symbol,
            }
        }
    }
}
