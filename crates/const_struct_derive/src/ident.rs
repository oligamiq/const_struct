use crate::const_struct_derive::{AbsolutePath, PathAndIdent};
use quote::format_ident;
use std::str::FromStr;
use strum::EnumString;
use syn::*;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, EnumString)]
pub enum PrimitiveIdent {
    U8,
    I8,
    Bool,
    U16,
    I16,
    U32,
    I32,
    F32,
    Char,
    U64,
    I64,
    F64,
    U128,
    I128,
    Usize,
    Isize,
}

pub enum AbsolutePathOrType {
    Path(AbsolutePath),
    Type(Box<dyn Fn(Expr) -> Type>),
}

pub fn get_primitive_ident_path(str: &str) -> Option<AbsolutePath> {
    PrimitiveIdent::from_str(str).ok().map(|_| {
        let new_path = format!("::const_struct::{}", str);
        AbsolutePath::new(parse_str(&new_path).unwrap())
    })
}

pub fn get_absolute_ident_path_from_ident(
    ident: &Path,
    addition: &Vec<PathAndIdent>,
) -> Option<AbsolutePathOrType> {
    for path_and_ident in addition {
        if path_and_ident.ident == *ident {
            return Some(AbsolutePathOrType::Path(path_and_ident.path.clone()));
        }
    }
    if let Some(ident) = ident.get_ident() {
        if let Some(_) = get_primitive_ident_path(&ident.to_string()) {
            return Some(AbsolutePathOrType::Type(Box::new(gen_primitive_ty(ident))));
        }
    }
    None
}

pub fn gen_primitive_ty(ident: &Ident) -> impl Fn(Expr) -> Type {
    // println!("ident: {:?}", ident);
    let base = match ident.to_string().as_str() {
        "U8" | "I8" | "Bool" => String::from("u8"),
        "U16" | "I16" => String::from("u16"),
        "U32" | "I32" | "F32" | "Char" => String::from("u32"),
        "U64" | "I64" | "F64" => String::from("u64"),
        "U128" | "I128" => String::from("u128"),
        "Usize" | "Isize" => String::from("usize"),
        _ => panic!("unknown primitive type"),
    };
    let base: Ident = format_ident!("{}", base);
    let name = ident.to_string().to_lowercase();
    let name: Ident = format_ident!("{}", name);
    let camel_name = format_ident!("{}Impl", ident);
    let expr_fn = move |expr: Expr| {
        let ty: Type = parse_quote! {
            ::const_struct::primitive::#camel_name::<{ unsafe { core::mem::transmute::<#name, #base>(#expr) } }>
        };
        // println!("ty: {:?}", ty);
        ty
    };
    expr_fn
}
