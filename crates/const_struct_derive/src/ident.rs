use crate::const_struct_derive::{AbsolutePath, PathAndIdent};
use std::str::FromStr;
use convert_case::{Case, Casing};
use quote::format_ident;
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

pub fn get_primitive_ident_path(str: &str) -> Option<AbsolutePath> {
    PrimitiveIdent::from_str(str).ok().map(|_| {
        let new_path = format!("::const_struct::{}", str);
        AbsolutePath::new(parse_str(&new_path).unwrap())
    })
}

pub fn get_absolute_ident_path_from_ident(
    ident: &Path,
    addition: &Vec<PathAndIdent>,
) -> Option<AbsolutePath> {
    for path_and_ident in addition {
        if path_and_ident.ident == *ident {
            return Some(path_and_ident.path.clone());
        }
    }
    if let Some(ident) = ident.get_ident() {
        if let Some(path) = get_primitive_ident_path(&ident.to_string()) {
            return Some(path);
        }
    }
    None
}

pub fn gen_primitive_ty(ident: &Ident) -> Type {
    let base = if ident.to_string().ends_with("size") {
        String::from("usize")
    } else {
        let size = core::mem::size_of::<usize>() * 8;
        format!("u{}", size)
    };
    let name = ident.to_string().from_case(Case::UpperCamel).to_case(Case::Snake);
    let camel_name = format_ident!("{}Impl", name);
    parse_quote! { ::const_struct::primitive::#camel_name::<{ unsafe { core::mem::transmute::<#name, #base>(($value)) } }> }
}
