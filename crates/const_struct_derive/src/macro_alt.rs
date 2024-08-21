use parse::{discouraged::Speculative as _, Parse, ParseStream};
use proc_macro2::TokenStream;
use quote::ToTokens as _;
use syn::*;

struct ExprAndExpr {
    expr_default: Expr,
    _comma: Token![,],
    is_under_score_expr: Expr,
}

impl Parse for ExprAndExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        let expr_default: Expr = fork.parse()?;
        let _comma: Token![,] = fork.parse()?;
        let is_under_score_expr: Expr = fork.parse()?;
        input.advance_to(&fork);
        Ok(Self {
            expr_default,
            _comma,
            is_under_score_expr,
        })
    }
}

pub fn match_underscore_alt(mac: Macro) -> TokenStream {
    if mac.path == parse_str::<Path>("::const_struct::match_underscore").unwrap() {
        let ExprAndExpr {
            expr_default,
            is_under_score_expr,
            ..
        } = parse2::<ExprAndExpr>(mac.tokens).unwrap();
        match expr_default {
            Expr::Infer(_) => is_under_score_expr,
            _ => expr_default,
        }
        .to_token_stream()
    } else {
        mac.to_token_stream()
    }
}
