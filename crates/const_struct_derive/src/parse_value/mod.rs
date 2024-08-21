use crate::{const_struct_derive::PathAndIdent, macro_alt::match_underscore_alt, rewriter::change_macro::Switcher};
use parse::{discouraged::Speculative as _, Parse, ParseStream};
use path::parse_value_path;
use proc_macro2::{Span, TokenStream};
use punctuated::Punctuated;
use syn::*;
use tuple::parse_value_tuple;

mod path;
mod struct_ty;
mod tuple;

pub struct AdditionDataArgs {
    pub _at: Token![@],
    pub _ident: Ident,
    pub _paren: token::Paren,
    pub data: Punctuated<PathAndIdent, Token![,]>,
}

#[derive(Debug)]
pub struct AdditionData {
    pub data: Vec<PathAndIdent>,
}

impl Into<AdditionData> for AdditionDataArgs {
    fn into(self) -> AdditionData {
        AdditionData {
            data: self.data.into_iter().collect(),
        }
    }
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
        // println!("at: {:?}", _at);
        // println!("fork: {}", fork);
        let _ident: Ident = fork.parse()?;
        // println!("ident: {:?}", _ident);
        // println!("fork: {}", fork);
        if _ident != "AdditionData" {
            return Err(Error::new_spanned(_ident, "expected `AdditionData`"));
        }
        let content;
        let _paren = parenthesized!(content in fork);
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
    pub additional_data: Option<AdditionDataArgs>,
    pub _comma: Option<Token![,]>,
    pub ty: Type,
    pub expr: Expr,
}

impl Parse for TyAndExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let additional_data = if input.peek(Token![@]) {
            Some(input.parse()?)
        } else {
            None
        };
        let _comma = if input.peek(Token![,]) {
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
            _comma,
        })
    }
}

pub fn parse_value_wrapper(input: TokenStream) -> Result<Type> {
    // println!("input: {}", input);
    // parse_value!((f32, u32), expr)
    let TyAndExpr {
        ty,
        expr,
        additional_data,
        ..
    } = syn::parse2(input)?;
    let additional_data = additional_data.unwrap_or_default();
    let additional_data: AdditionData = additional_data.into();
    // dbg!(&expr);

    let expr = expr.switcher(&match_underscore_alt);

    // dbg!(&additional_data);
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
