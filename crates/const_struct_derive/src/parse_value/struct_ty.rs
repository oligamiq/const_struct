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
    let struct_ident = struct_data.get_ty_ident();

    let absolute_struct_path = addition_data.get_changed_path(&parse_quote! { #struct_ident });

    // println!("absolute_struct_path: {}", absolute_struct_path.to_token_stream());

    if struct_data.label != Label::Struct {
        return Err(Error::new(struct_ident.span(), "This is not a struct type"));
    }

    // println!("struct_ident: {:?}", struct_ident);

    // let parsed_values = struct_data.get_parsed_values(expr.clone(), &info)?;

    // println!("parsed_values: {:?}", parsed_values);

    let gen_tys = struct_data.get_generics_types();

    // println!("gen_tys: {}", quote::quote! { #(#gen_tys),* });

    // {
    //     println!("info: ");
    //     let info = &info.correspondence;
    //     for (ident, type_or_expr) in info.iter() {
    //         println!("{}: {}", ident, quote::quote! { #type_or_expr });
    //     }
    // };

    let gen_tys = gen_tys
        .iter()
        .map(|ty| {
            // println!("ty__: {:?}", ty);

            let ty = info
                .correspondence
                .iter()
                .find_map(|(ident, type_or_expr)| match ty {
                    GenericParam::Const(const_param) => {
                        if const_param.ident == *ident {
                            if let TypeOrExpr::Expr(_) = type_or_expr {
                                // println!("const_param: {}", quote::quote! { #const_param });
                                // println!("type_or_expr: {}", quote::quote! { #type_or_expr });
                                Some(type_or_expr)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    GenericParam::Type(type_param) => {
                        // println!("ident_: {:?}", ident);
                        if type_param.ident == *ident {
                            if let TypeOrExpr::Type(_) = type_or_expr {
                                // println!("type_param: {:?}", type_param);
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
                    eprintln!("error: This function does not support Type::Infer");
                    unimplemented!()
                }

                GenericArgument::Type(ty.clone())
            }
            TypeOrExpr::Expr(inner_expr) => {
                if let Expr::Infer(_) = inner_expr {
                    // println!("num: {}", num);

                    let expr = gen_get_const_generics(
                        struct_data.const_fn.clone(),
                        ident_tys.clone(),
                        expr.clone(),
                        num,
                    );

                    if let Some(expr) = expr {
                        return GenericArgument::Const(expr);
                    } else {
                        eprintln!("error: This function does not support Expr::Infer");
                        unimplemented!()
                    }
                }

                GenericArgument::Const(inner_expr.clone())
            }
        })
        .collect::<Vec<GenericArgument>>();

    // println!("gen_tys: {}", quote::quote! { #(#gen_tys),* });

    let head_ty: Type = parse_quote! {
        #absolute_struct_path<#(#gen_tys),*>
    };

    // println!("head_ty: {}", head_ty.to_token_stream());

    let str_hash = addition_data
        .get_changed_path_from_quote(quote::quote! { ::const_struct::primitive::str_hash });
    // let primitive_traits = addition_data
    //     .get_changed_path_from_quote(quote::quote! { ::const_struct::primitive::PrimitiveTraits });
    let hash_bridge = addition_data
        .get_changed_path_from_quote(quote::quote! { ::const_struct::primitive::HashBridge });
    let hash_bridge_bridge = addition_data
        .get_changed_path_from_quote(quote::quote! { ::const_struct::primitive::HashBridgeBridge });
    let root_hash_bridge_ident = crate::root_hash_bridge_ident();
    #[cfg(not(feature = "rand_support"))]
    {
        let ty: Type = parse_quote! {
            #hash_bridge<{
                type T = #head_ty;

                const NAME_HASH: u64 = (#str_hash(stringify!(#expr)) as u32 as u64 + #hash as u32 as u64);

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
        return Ok(ty);
    }

    #[cfg(feature = "rand_support")]
    {
        let hash = rand::random::<u64>();
        let ty: Type = parse_quote! {
            #hash_bridge<{
                type T = #head_ty;

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
