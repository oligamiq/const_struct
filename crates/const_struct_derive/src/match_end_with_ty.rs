use parse::{discouraged::Speculative as _, Parse, ParseStream};
use proc_macro2::TokenStream;
use quote::*;
use syn::*;

use crate::util::is_end_with_ty;

pub struct ExprAndTypeAndExpr {
    compare_target: Expr,
    _comma: Token![,],
    ty: Type,
}

impl Parse for ExprAndTypeAndExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        let compare_target: Expr = fork.parse()?;
        let _comma: Token![,] = fork.parse()?;
        let ty: Type = fork.parse()?;
        input.advance_to(&fork);
        Ok(Self {
            compare_target,
            _comma,
            ty,
        })
    }
}

pub fn match_end_with_ty(input: TokenStream) -> Result<TokenStream> {
    let ExprAndTypeAndExpr {
        compare_target, ty, ..
    } = parse2::<ExprAndTypeAndExpr>(input)?;

    if let Ok(path) = parse2::<Path>(quote! { #compare_target }) {
        if is_end_with_ty(&path) {
            return Ok(quote! {
                #compare_target
            });
        }
    }

    Ok(quote! {
        #ty
    })
}
