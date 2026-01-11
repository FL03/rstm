/*
    appellation: impl_binary <module>
    authors: @FL03
*/
use crate::ast::{FiniteStateMachineAst, RuleAst};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

/// Procedural macro entry point
pub fn impl_wrapper_binary_ops(input: FiniteStateMachineAst) -> TokenStream {
    let base = generate_rules(&input);
    let assign = impl_assign_ops(&input);

    quote! {
        #(#base)*

        #(#assign)*
    }
}

fn generate_rules(
    FiniteStateMachineAst {
        target, field, ops, ..
    }: &FiniteStateMachineAst,
) -> Vec<TokenStream> {
    let mut impls = Vec::new();
    for rule in ops {
        let _impl = handle_rule(rule);
        impls.push(_impl);
    }
    impls
}
