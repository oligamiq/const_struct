use parse::{discouraged::Speculative as _, Parse, ParseStream};
use proc_macro2::TokenStream;
use punctuated::Punctuated;
use quote::ToTokens as _;
use syn::*;

use crate::{
    ident::{gen_option_ty, gen_primitive_ty},
    parse_value::{struct_ty::parse_value_struct_ty, AdditionData},
    util_macro::{self, GenericInfo, GenericsData, TypeOrExpr},
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
    _expr_or_type: Vec<TypeOrExpr>,
    value: Expr,
}

impl Parse for StructMacroAltArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        let mut _expr_or_type: Vec<TypeOrExpr> =
            Punctuated::<TypeOrExpr, Token![,]>::parse_terminated(&fork)?
                .into_iter()
                .collect();
        // for item in &expr_or_type {
        //     match item {
        //         TypeOrExpr::Type(_) => {
        //         }
        //         TypeOrExpr::Expr(_) => {
        //         }
        //     }
        // }
        let value = match _expr_or_type.pop() {
            Some(TypeOrExpr::Expr(expr)) => expr.clone(),
            _ => return Err(Error::new(fork.span(), "expected expr")),
        };
        input.advance_to(&fork);
        Ok(Self {
            _expr_or_type,
            value,
        })
    }
}

pub fn struct_macro_alt(
    addition_data: AdditionData,
    ident_tys: Vec<TokenStream>,
    data: GenericsData,
    new_generic: Vec<GenericArgument>,
    hash: u64,
) -> impl Fn(TokenStream) -> Result<Type> {
    if data.label != util_macro::Label::Struct {
        panic!("expected struct label for macro alt generation, got {:?}", data.label);
    }

    move |input: TokenStream| {
        let StructMacroAltArgs { value, .. } = parse2::<StructMacroAltArgs>(input)?;

        // let generic_info = data
        //     .get_generics_types()
        //     .into_iter()
        //     .zip(expr_or_type)
        //     .enumerate()
        //     .map(|(num, (ty, expr_or_type))| {
        //         let ident_and_expr_or_type = match ty {
        //             GenericParam::Type(ty) => {
        //                 if let TypeOrExpr::Type(Type::Infer(_)) = expr_or_type {
        //                     return Err(Error::new(ty.ident.span(), "expected type"));
        //                 }
        //                 Ok((ty.ident.clone(), expr_or_type))
        //             }
        //             GenericParam::Const(const_param) => {
        //                 if let TypeOrExpr::Expr(Expr::Infer(_)) = expr_or_type {
        //                     let expr_or_type =
        //                         gen_get_const_generics(data.const_fn.clone(), value.clone(), num);
        //                     if let Some(expr_or_type) = expr_or_type {
        //                         return Ok((
        //                             const_param.ident.clone(),
        //                             TypeOrExpr::Expr(expr_or_type),
        //                         ));
        //                     }
        //                 }
        //                 Ok((const_param.ident.clone(), expr_or_type))
        //             }
        //             _ => unimplemented!(),
        //         };
        //         ident_and_expr_or_type
        //     })
        //     .collect::<Result<Vec<_>>>()?;

        let generic_info = data
            .get_generics_types()
            .into_iter()
            .zip(new_generic.clone())
            .map(|(ty, expr_or_type)| match ty {
                GenericParam::Type(ty) => {
                    if let GenericArgument::Type(ty_) = expr_or_type {
                        Ok((ty.ident.clone(), TypeOrExpr::Type(ty_)))
                    } else {
                        Err(Error::new(
                            ty.ident.span(),
                            "`_` is not allowed in type position on inner declaration",
                        ))
                    }
                }
                GenericParam::Const(const_param) => {
                    if let GenericArgument::Const(expr) = expr_or_type {
                        Ok((const_param.ident.clone(), TypeOrExpr::Expr(expr)))
                    } else {
                        Err(Error::new(
                            const_param.ident.span(),
                            "`_` is not allowed in const position on inner declaration",
                        ))
                    }
                }
                _ => unimplemented!(),
            })
            .collect::<Result<Vec<_>>>()?;

        let parse_value_struct = parse_value_struct_ty(
            addition_data.clone(),
            ident_tys.clone(),
            data.clone(),
            GenericInfo {
                correspondence: generic_info,
            },
            value,
            hash,
        )?;

        Ok(parse_value_struct)
    }
}

pub fn default_primitive_macro_alt(mac: Macro) -> TokenStream {
    if let Some(ident) = mac.path.get_ident() {
        if let Some(ty) = gen_primitive_ty(ident) {
            ty(parse2(mac.tokens).unwrap()).to_token_stream()
        } else if let Some(ty) = gen_option_ty(ident) {
            ty(parse2(mac.tokens).unwrap()).to_token_stream()
        } else {
            mac.to_token_stream()
        }
    } else {
        mac.to_token_stream()
    }
}
