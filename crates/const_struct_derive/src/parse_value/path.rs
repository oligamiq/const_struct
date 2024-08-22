use crate::ident::AbsolutePathOrType;

use super::{parse_value, AdditionData};
use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::*;

pub fn parse_value_path(
    path: TypePath,
    expr: Expr,
    additional_data: &AdditionData,
) -> Result<Type> {
    let path = path.path;

    // dbg!(&path);

    // Option
    if path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.first().unwrap().ident.to_string() == "Option"
    {
        let args = &path.segments.first().unwrap().arguments;
        let args = match args {
            PathArguments::AngleBracketed(args) => args,
            _ => {
                return Err(Error::new_spanned(
                    path,
                    "expected angle bracketed arguments",
                ))
            }
        };
        let t = args.args.first().unwrap();
        let t = match t {
            GenericArgument::Type(t) => t,
            _ => return Err(Error::new_spanned(t, "expected type")),
        };
        let generic_expr_t: Expr = parse_quote!({
            let v0: #path = #expr;
            match v0 {
                Option::None => unsafe { core::mem::zeroed() },
                Option::Some(v0) => v0,
            }
        });
        let parsed_t = parse_value(t.clone(), generic_expr_t, additional_data)?;

        let ty: Type = parse_quote!(
            ::const_struct::primitive::EnumQueuePlaneHead<
                #path,
                ::const_struct::primitive::EnumQueuePlaneDataType<
                    #parsed_t,
                    ::const_struct::primitive::EnumQueuePlaneEnd,
                >,
                {
                    let v0: #path = #expr;
                    match v0 {
                        Option::None => 0,
                        Option::Some(_) => 1,
                    }
                },
            >
        );

        return Ok(ty);
    }

    let path = {
        // Convert the last segment to camel case
        let mut path = path;
        path.segments.last_mut().unwrap().ident = Ident::new(
            &path
                .segments
                .last()
                .unwrap()
                .ident
                .to_string()
                .to_case(Case::UpperCamel),
            Span::call_site(),
        );
        path
    };
    let path = additional_data.get_absolute_path(&path);

    let path = match path {
        AbsolutePathOrType::Path(path) => path,
        AbsolutePathOrType::Type(ty) => {
            return Ok(ty(expr));
        }
    };

    let path = path.path();

    // dbg!(&path);

    let (path_ident, path_arg) = {
        let mut path_ident = path.clone();
        let path_arg = &mut path_ident.segments.last_mut().unwrap().arguments;
        let path_arg_kept = path_arg.clone();
        *path_arg = PathArguments::None;
        (path_ident, path_arg_kept)
    };

    // println!("mac? {}", quote::quote!(#path_ident!(#expr)));

    match path_arg {
        PathArguments::None => {
            let mac: Macro = parse_quote! {
                #path_ident!(#expr)
            };

            Ok(Type::Macro(TypeMacro { mac }))
        }
        PathArguments::AngleBracketed(args) => {
            let args = args.args;
            let mac: Macro = parse_quote! {
                #path_ident!(#args, #expr)
            };

            Ok(Type::Macro(TypeMacro { mac }))
        }
        PathArguments::Parenthesized(_) => Err(Error::new_spanned(
            path,
            "expected angle bracketed arguments",
        )),
    }
}
