/*
    Appellation: rule_ast <module>
    Created At: 2026.01.11:11:41:13
    Contrib: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Impl, Paren};
use syn::{AngleBracketedGenericArguments, Ident, Token, WhereClause, braced};

pub struct HeadAst {    
    pub group: Paren,
    pub state: Ident,
    pub comma: Token![,],
    pub symbol: Ident,
}
#[allow(dead_code)]
/// The abstract syntax tree for a single operation rule;
/// 
/// ```no_run
/// (state, symbol) -> Direction(next_state, next_symbol)
/// ```
pub struct RuleAst {
    pub name: Ident,
    pub dot: Token![.],
    pub call: Ident,
}

impl Parse for HeadAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let group = syn::parenthesized!(content in input);
        let state = content.parse::<Ident>()?;
        let comma = content.parse::<Token![,]>()?;
        let symbol = content.parse::<Ident>()?;
        Ok(Self {
            group,
            state,
            comma,
            symbol,
        })
    }
}

impl Parse for RuleAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let period = input.parse::<Token![.]>()?;
        let call = input.parse::<Ident>()?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(Self {
            name,
            dot: period,
            call,
        })
    }
}

