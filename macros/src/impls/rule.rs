/*
    Appellation: rule <module>
    Created At: 2026.01.11:11:44:15
    Contrib: @FL03
*/
use crate::ast::{HeadAst, RuleAst};

use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_rule(ast: crate::ast::RuleAst) -> TokenStream {
    quote! {}
}
