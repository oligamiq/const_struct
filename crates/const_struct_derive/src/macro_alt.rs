use parse::{discouraged::Speculative as _, Parse, ParseStream};
use proc_macro2::TokenStream;
use punctuated::Punctuated;
use quote::ToTokens as _;
use syn::*;

use crate::{
    parse_value::struct_ty::parse_value_struct_ty,
    util_macro::{self, ConstOrType, GenericInfo, GenericsData, TypeOrExpr},
};

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

pub struct StructMacroAltArgs {
    expr_or_type: Vec<TypeOrExpr>,
    value: Expr,
}

impl Parse for StructMacroAltArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        println!("input: {}", input);
        let fork = input.fork();
        let mut expr_or_type: Vec<TypeOrExpr> =
            Punctuated::<TypeOrExpr, Token![,]>::parse_terminated(&fork)?
                .into_iter()
                .collect();
        println!("expr_or_type: {}", quote::quote! { #(#expr_or_type),* });
        for item in &expr_or_type {
            match item {
                TypeOrExpr::Type(_) => {
                    println!("TypeOrExpr::Type");
                }
                TypeOrExpr::Expr(_) => {
                    println!("TypeOrExpr::Expr");
                }
            }
        }
        let value = match expr_or_type.pop() {
            Some(TypeOrExpr::Expr(expr)) => {
                expr.clone()
            }
            _ => return Err(Error::new(fork.span(), "expected expr")),
        };
        input.advance_to(&fork);
        Ok(Self {
            expr_or_type,
            value,
        })
    }
}

pub fn struct_macro_alt(data: GenericsData) -> impl Fn(TokenStream) -> Result<Type> {
    if data.label != util_macro::Label::Struct {
        panic!("Expected struct");
    }
    let const_or_type = data.const_or_type();
    let struct_macro = move |input: TokenStream| {
        let StructMacroAltArgs {
            mut expr_or_type,
            value,
        } = parse2::<StructMacroAltArgs>(input)?;

        println!("expr_or_type: {}", quote::quote! { #(#expr_or_type),* });
        println!("value: {:?}", value);
        println!("const_or_type: {:?}", const_or_type);

        // check
        for (expr_or_type, ty) in expr_or_type.iter_mut().zip(const_or_type.iter()) {
            match (&expr_or_type, ty) {
                (TypeOrExpr::Type(_), ConstOrType::Type) => {}
                (TypeOrExpr::Expr(_), ConstOrType::Const) => {}
                (TypeOrExpr::Type(ty), ConstOrType::Const) => {
                    println!("ty: {}", quote::quote! { #ty });
                    let expr: Expr = parse_quote!(#ty);
                    *expr_or_type = TypeOrExpr::Expr(expr);
                }
                (TypeOrExpr::Expr(expr), ConstOrType::Type) => {
                    println!("expr: {:?}", quote::quote! { #expr });
                    match parse2::<Type>(quote::quote!(expr.clone())) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("error: {:?}", e);
                        }
                    }
                    let ty: Type = parse_quote!(#expr);
                    *expr_or_type = TypeOrExpr::Type(ty);
                }
            }
        }

        let generic_info = data
            .get_generics_types()
            .into_iter()
            .zip(expr_or_type)
            .map(|(ty, expr_or_type)| {
                let ident = match ty {
                    GenericParam::Type(ty) => ty.ident.clone(),
                    GenericParam::Const(const_param) => const_param.ident.clone(),
                    _ => unimplemented!(),
                };
                (ident, expr_or_type)
            })
            .collect::<Vec<_>>();

        let parse_value_struct = parse_value_struct_ty(
            data.clone(),
            GenericInfo {
                correspondence: generic_info,
            },
            value,
        )?;

        Ok(parse_value_struct)
    };

    struct_macro
}
