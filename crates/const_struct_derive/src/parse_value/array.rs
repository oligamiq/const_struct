use syn::*;

use super::AdditionData;

pub fn parse_value_array(
    array: TypeArray,
    expr: Expr,
    additional_data: &AdditionData,
) -> Result<Type> {
    let len = array.len;
    if let Expr::Lit(ExprLit {
        lit: Lit::Int(lit), ..
    }) = len
    {
        let len = lit.base10_parse::<usize>()?;

        dbg!(&len);
    }

    todo!()
}
