use parse::{Parse, ParseStream};
use proc_macro::Punct;
use proc_macro2::{Span, TokenStream};
use punctuated::Punctuated;
use syn::*;

pub struct TyAndExpr {
    ty: Type,
    expr: Expr,
}

impl Parse for TyAndExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let expr: Expr = input.parse()?;
        Ok(Self { ty, expr })
    }
}

pub fn parse_value_wrapper(input: TokenStream) -> Result<Type> {
    // parse_value!((f32, u32), expr)
    let TyAndExpr { ty, expr } = syn::parse2(input)?;
    parse_value(ty, expr)
}

pub fn parse_value(input: Type, expr: Expr) -> Result<Type> {
    match input {
        Type::Tuple(tuple) => parse_value_tuple(tuple, expr),
        Type::Path(path) => parse_value_path(path, expr),
        _ => Err(Error::new_spanned(input, "unsupported type")),
    }
}

pub fn parse_value_tuple(tuple: TypeTuple, expr: Expr) -> Result<Type> {
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
            let parsed_ty = parse_value(ty, expr);
            parsed_ty
        })
        .collect::<Result<Punctuated<Type, Token![,]>>>()?;

    Ok(Type::Tuple(TypeTuple { elems, ..tuple }))
}

pub fn parse_value_path(path: TypePath, expr: Expr) -> Result<Type> {
    dbg!(&path);
    dbg!(expr);

    Ok(Type::Path(path))
}
