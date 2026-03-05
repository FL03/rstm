/*
    Appellation: rule_ast <module>
    Created At: 2026.01.11:11:41:13
    Contrib: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{Expr, Ident, Token, braced, parenthesized};

/// The abstract syntax tree for the head of a rule / Turing machine: `(state, symbol)`
#[derive(Clone)]
pub struct HeadAst {
    #[allow(dead_code)]
    pub group: Paren,
    pub state: Expr,
    #[allow(dead_code)]
    pub comma: Token![,],
    pub symbol: Expr,
}
/// The abstract syntax tree for the tail of a rule / Turing machine: `{Left|Right|Stay}(state, symbol)`
#[derive(Clone)]
pub struct TailAst {
    pub direction: Ident,
    #[allow(dead_code)]
    pub group: Paren,
    pub next_state: Expr,
    #[allow(dead_code)]
    pub comma: Token![,],
    pub next_symbol: Expr,
}

/// The abstract syntax tree for a single operation rule
///
/// `(state, symbol) -> Direction(next_state, next_symbol)`
#[derive(Clone)]
pub struct RuleAst {
    pub head: HeadAst,
    #[allow(dead_code)]
    pub rarrow: Token![->],
    pub tail: TailAst,
}

pub struct RuleBlockAst {
    #[allow(dead_code)]
    pub key: crate::keywords::rules,
    #[allow(dead_code)]
    pub colon: Token![:],
    pub rules: Punctuated<RuleAst, Token![,]>,
    #[allow(dead_code)]
    pub semi: Option<Token![;]>,
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
                    next_state,
                    next_symbol,
                    ..
                },
            ..
        } = self;
        // create a rule
        quote::quote! {
            rstm::Rule {
                head: rstm::Head {
                    state: #state,
                    symbol: #symbol,
                },
                tail: rstm::Tail {
                    direction: rstm::Direction::#direction,
                    next_state: #next_state,
                    write_symbol: #next_symbol,
                }
            }
        }
    }
}

impl Parse for HeadAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let group = parenthesized! { content in input };
        let state = content.parse::<Expr>()?;
        let comma = content.parse::<Token![,]>()?;
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
        let content;
        let group = parenthesized! { content in input };
        let next_state = content.parse::<Expr>()?;
        let comma = content.parse::<Token![,]>()?;
        let next_symbol = content.parse::<Expr>()?;
        Ok(Self {
            direction,
            next_state,
            next_symbol,
            group,
            comma,
        })
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

impl Parse for RuleBlockAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse::<crate::keywords::rules>()?;
        let colon = input.parse::<Token![:]>()?;
        let content;
        let _ = braced! { content in input };
        let rules = content.parse_terminated(RuleAst::parse, Token![,])?;
        // if present, consume the optional semicolon after the rules block
        let semi = if content.peek(Token![;]) {
            Some(content.parse()?)
        } else {
            None
        };
        Ok(Self {
            key,
            colon,
            rules,
            semi,
        })
    }
}
