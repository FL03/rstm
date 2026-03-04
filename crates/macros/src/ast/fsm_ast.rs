/*
    appellation: fsm_ast <module>
    authors: @FL03
*/
use crate::ast::RulesBlockAst;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

/// Parses: `default_state: <expr>;`
pub struct DefaultStateFieldAst {
    #[allow(dead_code)]
    pub key: crate::keywords::default_state,
    #[allow(dead_code)]
    pub colon: Token![:],
    pub state: Expr,
    #[allow(dead_code)]
    pub semi: Option<Token![;]>,
}

/// The abstract syntax tree for the `fsm!` procedural macro.
///
/// Syntax:
/// ```ignore
/// fsm! {
///     default_state: <expr>;
///     rules: {
///         (state, symbol) -> Direction(next_state, next_symbol),
///         ...
///     };
/// }
/// ```
pub struct FiniteStateMachineAst {
    pub default_state: Option<DefaultStateFieldAst>,
    pub rules: RulesBlockAst,
}

/*
 ************* Implementations *************
*/

impl Parse for DefaultStateFieldAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let colon = input.parse()?;
        let state = input.parse()?;
        // optionally consume a trailing semicolon after the default state declaration
        let semi = if input.peek(Token![;]) {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self {
            key,
            colon,
            state,
            semi, // optional semicolon, consume if present
        })
    }
}

impl Parse for FiniteStateMachineAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // optionally parse `default_state: <expr>;`
        let default_state = if input.peek(crate::keywords::default_state) {
            Some(input.parse::<DefaultStateFieldAst>()?)
        } else {
            None
        };
        // parse the required `rules: { ... }` block
        let rules = input.parse::<RulesBlockAst>()?;
        Ok(Self {
            default_state,
            rules,
        })
    }
}

fn _parse_default_state(input: ParseStream) -> Option<DefaultStateFieldAst> {
    if input.peek(Token![#]) && input.peek3(crate::keywords::default_state) {
        return input.parse::<DefaultStateFieldAst>().ok();
    }
    None
}
