use crate::const_struct_derive::{AbsolutePath, PathAndIdent};
use proc_macro2::TokenStream;
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
        if get_primitive_ident_path(&ident.to_string()).is_some() {
            return Some(AbsolutePathOrType::Type(Box::new(
                gen_primitive_ty(ident).unwrap(),
            )));
        }
    }
    None
}

pub fn gen_primitive_ty(ident: &Ident) -> Option<impl Fn(Expr) -> Type> {
    // println!("ident: {:?}", ident);
    let base = match ident.to_string().as_str() {
        "U8" | "I8" | "Bool" => String::from("u8"),
        "U16" | "I16" => String::from("u16"),
        "U32" | "I32" | "F32" | "Char" => String::from("u32"),
        "U64" | "I64" | "F64" => String::from("u64"),
        "U128" | "I128" => String::from("u128"),
        "Usize" | "Isize" => String::from("usize"),
        _ => return None,
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
    Some(expr_fn)
}

pub fn gen_option_ty(ident: &Ident) -> Option<impl Fn(TokenStream) -> Type> {
    let is_some: bool = match ident.to_string().as_str() {
        "Some" => true,
        "None" => false,
        _ => return None,
    };
    let expr_fn = move |stream: TokenStream| {
        if is_some {
            let ty: Type = parse2::<Type>(stream).unwrap_or_else(|_| {
                eprintln!("error: Some! expects a type");
                unimplemented!()
            });
            let ty: Type = parse_quote! {
                ::const_struct::primitive::SomeImpl<#ty>
            };
            ty
        } else {
            // streamは空っぽのはず
            if !stream.is_empty() {
                eprintln!("error: None! does not have any arguments");
                unimplemented!()
            }
            let ty: Type = parse_quote! {
                ::const_struct::primitive::NoneImpl
            };
            ty
        }
    };
    Some(expr_fn)
}
