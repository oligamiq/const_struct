use crate::{
    util::gen_get_const_generics,
    util_macro::{GenericInfo, GenericsData, Label, TypeOrExpr},
};
use proc_macro2::TokenStream;
use syn::*;

use super::AdditionData;

// use quote::ToTokens as _;

/// _ は、GenericInfoを作成するときに考慮する
#[allow(unused_variables)]
pub fn parse_value_struct_ty(
    addition_data: AdditionData,
    ident_tys: Vec<TokenStream>,
    struct_data: GenericsData,
    info: GenericInfo,
    expr: Expr,
    hash: u64,
) -> Result<Type> {
    let struct_ident = struct_data.get_ty_ident()?;

    let absolute_struct_path = addition_data.get_changed_path(&parse_quote! { #struct_ident })?;

    if struct_data.label != Label::Struct {
        return Err(Error::new(struct_ident.span(), "This is not a struct type"));
    }

    let gen_tys = struct_data.get_generics_types();

    let gen_tys = gen_tys
        .iter()
        .map(|ty| {
            let ty = info
                .correspondence
                .iter()
                .find_map(|(ident, type_or_expr)| match ty {
                    GenericParam::Const(const_param) => {
                        if const_param.ident == *ident {
                            if let TypeOrExpr::Expr(_) = type_or_expr {
                                Some(type_or_expr)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    GenericParam::Type(type_param) => {
                        if type_param.ident == *ident {
                            if let TypeOrExpr::Type(_) = type_or_expr {
                                Some(type_or_expr)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    _ => unimplemented!(),
                });
            ty.unwrap_or(&TypeOrExpr::Expr(parse_quote! { _ })).clone()
        })
        .enumerate()
        .map(|(num, ty_or_expr)| match ty_or_expr {
            TypeOrExpr::Type(ty) => {
                if let Type::Infer(_) = ty {
                    return Err(Error::new(
                        proc_macro2::Span::call_site(),
                        "Type::Infer (`_`) is not supported in this context",
                    ));
                }

                Ok(GenericArgument::Type(ty.clone()))
            }
            TypeOrExpr::Expr(inner_expr) => {
                if let Expr::Infer(_) = inner_expr {

                    let expr = gen_get_const_generics(
                        struct_data.const_fn.clone(),
                        ident_tys.clone(),
                        expr.clone(),
                        num,
                    );

                    if let Some(expr) = expr {
                        return Ok(GenericArgument::Const(expr));
                    } else {
                        return Err(Error::new(
                            proc_macro2::Span::call_site(),
                            "Expr::Infer (`_`) could not be resolved for const generic",
                        ));
                    }
                }

                Ok(GenericArgument::Const(inner_expr.clone()))
            }
        })
        .collect::<Result<Vec<GenericArgument>>>()?;

    let head_ty: Type = parse_quote! {
        #absolute_struct_path<#(#gen_tys),*>
    };

    let str_hash = addition_data
        .get_changed_path_from_quote(quote::quote! { ::const_struct::primitive::str_hash })?;
    let hash_bridge = addition_data
        .get_changed_path_from_quote(quote::quote! { ::const_struct::primitive::HashBridge })?;
    let hash_bridge_bridge = addition_data
        .get_changed_path_from_quote(quote::quote! { ::const_struct::primitive::HashBridgeBridge })?;
    let root_hash_bridge_ident = crate::root_hash_bridge_ident();
    #[cfg(not(feature = "rand_support"))]
    {
        let ty: Type = parse_quote! {
            #hash_bridge<{
                type T = #head_ty;

                const NAME_HASH: u64 = (#str_hash(stringify!(#expr)) as u32 as u64 + #hash as u32 as u64);

                #[allow(non_local_definitions)]
                impl #hash_bridge_bridge<NAME_HASH, {#str_hash(file!())}, {column!()}, {line!()}> for #root_hash_bridge_ident<NAME_HASH, {#str_hash(file!())}, {column!()}, {line!()}> {
                    type DATATYPE = T;
                    const DATA: Self::DATATYPE = #expr;
                }

                NAME_HASH
            }, {
                #str_hash(file!())
            }, {
                column!()
            }, {
                line!()
            },
            #root_hash_bridge_ident<{ (#str_hash(stringify!(#expr)) as u32 as u64 + #hash as u32 as u64) }, {#str_hash(file!())}, {column!()}, {line!()}>
            >
        };
        Ok(ty)
    }

    #[cfg(feature = "rand_support")]
    {
        let hash = rand::random::<u64>();
        let ty: Type = parse_quote! {
            #hash_bridge<{
                type T = #head_ty;

                #[allow(non_local_definitions)]
                impl #hash_bridge_bridge<#hash, {#str_hash(file!())}, {column!()}, {line!()}> for #root_hash_bridge_ident<#hash, {#str_hash(file!())}, {column!()}, {line!()}> {
                    type DATATYPE = T;
                    const DATA: Self::DATATYPE = #expr;
                }

                #hash
            }, {
                #str_hash(file!())
            }, {
                column!()
            }, {
                line!()
            },
            #root_hash_bridge_ident<{ #hash }, {#str_hash(file!())}, {column!()}, {line!()}>
            >
        };
        return Ok(ty);
    };
}
