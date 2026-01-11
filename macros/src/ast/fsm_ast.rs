/*
    appellation: ops <module>
    authors: @FL03
*/
use super::RuleAst;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Impl, Paren};
use syn::{AngleBracketedGenericArguments, Ident, Token, WhereClause, braced};

fn _parse_ops(input: ParseStream) -> syn::Result<Punctuated<RuleAst, Token![,]>> {
    // parse the operations defined within braces
    let content;
    let _ = braced! { content in input };
    Punctuated::parse_terminated(&content)
}

#[allow(dead_code)]
/// The abstract syntax tree for the `fsm!` procedural macro
pub struct FiniteStateMachineAst {
    pub impl_token: Impl,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub target: Ident,
    pub field: Option<Ident>,
    pub where_clause: Option<WhereClause>,
    pub ops: Punctuated<RuleAst, Token![,]>,
}

/*
 ************* Implementations *************
*/

impl Parse for FiniteStateMachineAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // parse the `impl` keyword
        let impl_token = input.parse::<Impl>()?;
        // detect any generic parameters
        let generics = if input.peek(Token![<]) {
            input.parse().ok()
        } else {
            None
        };
        let target = input.parse::<Ident>()?;
        // resolve the optional named field
        let field = if input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        // parse the optional where clause
        let where_clause = if input.peek(Token![where]) {
            Some(input.parse()?)
        } else {
            None
        };
        // parse the operations block
        let content;
        let _ = braced! { content in input };
        let mut ops = Punctuated::new();
        while !content.is_empty() {
            ops.push(content.parse::<RuleAst>()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            impl_token,
            generics,
            target,
            field,
            where_clause,
            ops,
        })
    }
}
