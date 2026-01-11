/*
    appellation: impl_binary <module>
    authors: @FL03
*/
use crate::ast::{FiniteStateMachineAst, RuleAst};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

fn handle_rule(rule: &RuleAst) -> TokenStream {
    let RuleAst { name, .. } = rule;
    let fn_name = format_ident!("{}", name);
    quote! {
        pub fn #fn_name() {
            // implementation goes here
        }
    }
}
