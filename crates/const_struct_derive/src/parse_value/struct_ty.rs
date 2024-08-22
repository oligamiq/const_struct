use crate::util_macro::{GenericInfo, GenericsData, Label, TypeOrExpr};
use syn::*;

/// _ は、GenericInfoを作成するときに考慮する
pub fn parse_value_struct_ty(
    struct_data: GenericsData,
    info: GenericInfo,
    expr: Expr,
) -> Result<Type> {
    let struct_ident = struct_data.get_ty_ident();

    if struct_data.label != Label::Struct {
        return Err(Error::new(struct_ident.span(), "This is not a struct type"));
    }

    // println!("struct_ident: {:?}", struct_ident);

    let parsed_values = struct_data.get_parsed_values(expr, &info)?;

    // println!("parsed_values: {:?}", parsed_values);

    let gen_tys = struct_data.get_generics_types();

    // println!("gen_tys: {:?}", gen_tys);

    // println!("info: {:?}", info);

    let gen_tys = gen_tys
        .iter()
        .map(|ty| {
            println!("ty__: {:?}", ty);

            let ty = info
            .correspondence
            .iter()
            .find_map(|(ident, type_or_expr)| match ty {
                GenericParam::Const(const_param) => {
                    if const_param.ident == *ident {
                        if let TypeOrExpr::Expr(_) = type_or_expr {
                            println!("const_param: {:?}", type_or_expr);
                            Some(type_or_expr)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                GenericParam::Type(type_param) => {
                        println!("ident_: {:?}", ident);
                        if type_param.ident == *ident {
                            if let TypeOrExpr::Type(_) = type_or_expr {
                                println!("type_param: {:?}", type_param);
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
            ty.expect("ty not found")
        })
        .map(|ty_or_expr| match ty_or_expr {
            TypeOrExpr::Type(ty) => {
                if let Type::Infer(_) = ty {
                    eprintln!("error: This function does not support Type::Infer");
                    unimplemented!()
                }

                GenericArgument::Type(ty.clone())
            },
            TypeOrExpr::Expr(expr) => {
                if let Expr::Infer(_) = expr {
                    eprintln!("error: This function does not support Expr::Infer");
                    unimplemented!()
                }

                GenericArgument::Const(expr.clone())
            }
        })
        .collect::<Vec<GenericArgument>>();

    let head_ty: Type = parse_quote! {
        #struct_ident<#(#gen_tys),*>
    };

    let ty = gen_value_struct_ty(head_ty, parsed_values);

    Ok(ty)
}

pub fn gen_value_struct_ty(head_ty: Type, queue_ty: Vec<Type>) -> Type {
    let queue_ty_rev = queue_ty.iter().rev();
    let mut ty: Type = parse_quote!(::const_struct::primitive::ConstStructPrimEnd);
    for queue_ty in queue_ty_rev {
        ty = parse_quote!(::const_struct::primitive::ConstStructPrimQueue<#queue_ty, #ty>);
    }
    let ty: Type = parse_quote! {
        ::const_struct::primitive::ConstStructPrimQueue<#head_ty, #ty>
    };
    ty
}
