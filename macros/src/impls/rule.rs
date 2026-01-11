/*
    Appellation: rule <module>
    Created At: 2026.01.11:11:44:15
    Contrib: @FL03
*/
use crate::ast::{HeadAst, RuleAst};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_rule(rule: &RuleAst) -> TokenStream {
    handle_rule(rule)
}


fn handle_rule(RuleAst {
        head: HeadAst { state, symbol, .. },
        ..
    }: &RuleAst) -> TokenStream {
    quote! {
        rstm::Rule {
            head: rstm::Head {
                state: rstm::State(#state),
                symbol: #symbol,
            },
            tail: rstm::Tail {
                new_state: rstm::State(#state),
                new_symbol: #symbol,
                direction: rstm::Direction::Stay,
            }
        }
    }
}
