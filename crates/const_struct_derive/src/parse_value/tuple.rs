use super::{parse_value, AdditionData};
use proc_macro2::Span;
use syn::{punctuated::Punctuated, *};

pub fn parse_value_tuple(
    tuple: TypeTuple,
    expr: Expr,
    additional_data: &AdditionData,
) -> Result<Type> {
    let TypeTuple { elems, .. } = tuple.clone();
    let elems = elems
        .into_iter()
        .enumerate()
        .map(|(num, ty)| {
            let expr: Expr = parse_quote!({
                let v0: #tuple = #expr;
                v0
            });
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
