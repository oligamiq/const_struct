use crate::const_struct_derive::PathAndIdent;
use convert_case::{Case, Casing};
use parse::{discouraged::Speculative as _, Parse, ParseStream};
use proc_macro::Punct;
use proc_macro2::{Span, TokenStream};
use punctuated::Punctuated;
use syn::*;

pub struct AdditionDataArgs {
    pub _at: Token![@],
    pub ident: Ident,
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
            ident: Ident::new("AdditionData", Span::call_site()),
            _paren: Default::default(),
            data: Default::default(),
        }
    }
}

impl Parse for AdditionDataArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        let _at: Token![@] = fork.parse()?;
        let ident: Ident = fork.parse()?;
        if ident != "AdditionData" {
            return Err(Error::new_spanned(ident, "expected `AdditionData`"));
        }
        let content;
        let _paren = parenthesized!(content in input);
        let data = Punctuated::parse_terminated(&content)?;
        input.advance_to(&fork);
        Ok(Self {
            _at,
            ident,
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
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let expr: Expr = input.parse()?;
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

pub fn parse_value_tuple(
    tuple: TypeTuple,
    expr: Expr,
    additional_data: &AdditionData,
) -> Result<Type> {
    let TypeTuple { elems, .. } = tuple;
    let elems = elems
        .into_iter()
        .enumerate()
        .map(|(num, ty)| {
            let expr = Expr::Field(ExprField {
                attrs: vec![],
                base: Box::new(expr.clone()),
                dot_token: Default::default(),
                member: Member::Unnamed(Index {
                    index: num as u32,
                    span: Span::call_site(),
                }),
            });
            let expr: Expr = parse_quote!({ #expr });
            let parsed_ty = parse_value(ty, expr, additional_data);
            parsed_ty
        })
        .collect::<Result<Punctuated<Type, Token![,]>>>()?;

    Ok(Type::Tuple(TypeTuple { elems, ..tuple }))
}

#[inline]
pub fn get_absolute_path(
    path: &Path,
    additional_data: &AdditionData,
) -> crate::const_struct_derive::AbsolutePath {
    crate::const_struct_derive::ConstStructAttr::get_absolute_path_inner(
        path,
        &additional_data.data,
    )
}

pub fn parse_value_path(
    path: TypePath,
    expr: Expr,
    additional_data: &AdditionData,
) -> Result<Type> {
    let path = path.path;
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
    let path = get_absolute_path(&path, additional_data);
    let path = path.path();

    // Option
    if path.is_ident("Option") {
    }

    let mac: Macro = parse_quote! {
        #path!(#expr)
    };

    Ok(Type::Macro(TypeMacro { mac }))
}
