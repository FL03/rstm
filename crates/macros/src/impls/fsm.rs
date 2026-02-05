/*
    appellation: impl_binary <module>
    authors: @FL03
*/
use crate::ast::{FiniteStateMachineAst, HeadAst, RuleAst, TailAst};
use proc_macro2::TokenStream;
use quote::quote;

/// Procedural macro entry point
pub fn impl_wrapper_binary_ops(input: FiniteStateMachineAst) -> TokenStream {
    let rules = generate_rules(&input);

    quote! {
        #(#rules)*
    }
}

fn generate_rules(FiniteStateMachineAst { ops, .. }: &FiniteStateMachineAst) -> Vec<TokenStream> {
    let mut impls = Vec::new();
    for rule in ops {
        let _impl = handle_rule(rule);
        impls.push(_impl);
    }
    impls
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
    quote! {
        rstm::Rule {
            head: rstm::Head {
                state: rstm::State(#state),
                symbol: #symbol,
            },
            tail: rstm::Tail {
                new_state: rstm::State(#next_state),
                new_symbol: #next_symbol,
                direction: rstm::Direction::#direction,
            }
        }
    }
}
