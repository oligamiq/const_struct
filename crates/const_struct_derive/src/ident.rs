use strum::EnumString;
use std::str::FromStr;
use crate::const_struct_derive::{DollarPath, PathAndIdent};
use syn::*;

#[derive(Debug, PartialEq, EnumString)]
pub enum PrimitiveIdent {
    u8,
    U8,
    i8,
    I8,
    bool,
    Bool,
    u16,
    U16,
    i16,
    I16,
    u32,
    U32,
    i32,
    I32,
    f32,
    F32,
    char,
    Char,
    u64,
    U64,
    i64,
    I64,
    f64,
    F64,
    u128,
    U128,
    i128,
    I128,
    usize,
    Usize,
    isize,
    Isize,
}

pub fn get_primitive_ident_path(str: &str) -> Option<DollarPath> {
    PrimitiveIdent::from_str(str).ok().map(|_| {
        let new_path = format!("::const_struct::primitive::{}", str);
        DollarPath {
            meta_dollar: None,
            path: parse_str(&new_path).unwrap(),
        }
    })
}

pub fn get_absolute_ident_path_from_ident(ident: &Ident, addition: Vec<PathAndIdent>) -> Option<DollarPath> {
    if let Some(path) = get_primitive_ident_path(&ident.to_string()) {
        return Some(path);
    }
    for path_and_ident in addition {
        if path_and_ident.ident == *ident {
            return Some(path_and_ident.path);
        }
    }
    None
}
