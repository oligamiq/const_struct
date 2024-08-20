use crate::const_struct_derive::PathAndIdent;
use parse::{discouraged::Speculative as _, Parse, ParseStream};
use path::parse_value_path;
use proc_macro2::{Span, TokenStream};
use punctuated::Punctuated;
use syn::*;
use tuple::parse_value_tuple;

mod path;
mod tuple;

pub struct AdditionDataArgs {
    pub _at: Token![@],
    pub _ident: Ident,
    pub _paren: token::Paren,
    pub data: Punctuated<PathAndIdent, Token![,]>,
}

pub struct AdditionData {
    pub data: Vec<PathAndIdent>,
}

impl Default for AdditionDataArgs {
    fn default() -> Self {
        Self {
            _at: Default::default(),
            _ident: Ident::new("AdditionData", Span::call_site()),
            _paren: Default::default(),
            data: Default::default(),
        }
    }
}

impl Parse for AdditionDataArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        let _at: Token![@] = fork.parse()?;
        let _ident: Ident = fork.parse()?;
        if _ident != "AdditionData" {
            return Err(Error::new_spanned(_ident, "expected `AdditionData`"));
        }
        let content;
        let _paren = parenthesized!(content in input);
        let data = Punctuated::parse_terminated(&content)?;
        input.advance_to(&fork);
        Ok(Self {
            _at,
            _ident,
            _paren,
            data,
        })
    }
}

pub struct TyAndExpr {
    additional_data: Option<AdditionDataArgs>,
    ty: Type,
    expr: Expr,
}

impl Parse for TyAndExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let additional_data = if input.peek(Token![@]) {
            Some(input.parse()?)
        } else {
            None
        };
        let fork = input.fork();
        let ty: Type = fork.parse()?;
        fork.parse::<Token![,]>()?;
        let expr: Expr = fork.parse()?;
        input.advance_to(&fork);
        Ok(Self {
            ty,
            expr,
            additional_data,
        })
    }
}

pub fn parse_value_wrapper(input: TokenStream) -> Result<Type> {
    // parse_value!((f32, u32), expr)
    let TyAndExpr {
        ty,
        expr,
        additional_data,
    } = syn::parse2(input)?;
    let additional_data = additional_data.unwrap_or_default();
    let additional_data = AdditionData {
        data: additional_data.data.into_iter().collect(),
    };
    dbg!(&ty);
    parse_value(ty, expr, &additional_data)
}

pub fn parse_value(input: Type, expr: Expr, additional_data: &AdditionData) -> Result<Type> {
    match input {
        Type::Tuple(tuple) => parse_value_tuple(tuple, expr, additional_data),
        Type::Path(path) => parse_value_path(path, expr, additional_data),
        // Type::
        _ => Err(Error::new_spanned(input, "unsupported type")),
    }
}
