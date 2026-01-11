/*
    Appellation: rule <module>
    Created At: 2026.01.11:11:44:15
    Contrib: @FL03
*/
use crate::ast::{HeadAst, RuleAst, TailAst};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_rule(rule: &RuleAst) -> TokenStream {
    handle_rule(rule)
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
            head: rstm::Head::new(#state, #symbol),
            tail: rstm::Tail::new(rstm::Direction::#direction, #next_state, #next_symbol)
        }
    }
}
