/*
    Appellation: rule_ast <module>
    Created At: 2026.01.11:11:41:13
    Contrib: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma, Paren};
use syn::{Expr, Ident, Token, parenthesized};
#[allow(dead_code)]
/// The abstract syntax tree for the head of a rule / Turing machine: `(state, symbol)`
pub struct HeadAst {
    pub group: Paren,
    pub state: Expr,
    pub comma: Comma,
    pub symbol: Expr,
}
#[allow(dead_code)]
/// The abstract syntax tree for the head of a rule / Turing machine: `Direction(state, symbol)`
pub struct TailAst {
    pub direction: Ident,
    pub head: HeadAst,
}
#[allow(dead_code)]
/// The abstract syntax tree for a single operation rule
///
/// `(state, symbol) -> Direction(next_state, next_symbol)`
pub struct RuleAst {
    pub head: HeadAst,
    pub rarrow: Token![->],
    pub tail: TailAst,
}

/*
 ************* Implementations *************
*/

impl RuleAst {
    #[allow(dead_code)]
    /// returns the string representation of the rule
    pub fn as_token_stream(&self) -> proc_macro2::TokenStream {
        // deconstruct the rule ast
        let RuleAst {
            head: HeadAst { state, symbol, .. },
            tail:
                TailAst {
                    direction,
                    head:
                        HeadAst {
                            state: next_state,
                            symbol: next_symbol,
                            ..
                        },
                },
            ..
        } = self;
        // create a rule
        quote::quote! {
            rstm::Rule {
                head: rstm::Head::new(#state, #symbol),
                tail: rstm::Tail::new(#next_state, #next_symbol, rstm::Direction::#direction)
            }
        }
    }
}

impl Parse for HeadAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let group = parenthesized! { content in input };
        let state = content.parse::<Expr>()?;
        let comma = content.parse::<Comma>()?;
        let symbol = content.parse::<Expr>()?;
        Ok(Self {
            group,
            state,
            comma,
            symbol,
        })
    }
}

impl Parse for TailAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let direction = input.parse::<Ident>()?;
        let head = input.parse::<HeadAst>()?;
        Ok(Self { direction, head })
    }
}

impl Parse for RuleAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let head = input.parse::<HeadAst>()?;
        let rarrow = input.parse::<Token![->]>()?;
        let tail = input.parse::<TailAst>()?;
        Ok(Self { head, rarrow, tail })
    }
}
