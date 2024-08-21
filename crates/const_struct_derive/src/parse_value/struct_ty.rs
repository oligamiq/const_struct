use crate::util_macro::{GenericsData, Label};
use syn::*;

pub fn parse_value_struct_ty(struct_data: GenericsData) -> Result<Type> {
    let GenericsData {
        _at,
        ident,
        _paren_token,
        label,
        _comma,
        const_fn,
        _comma2,
        macros,
    } = struct_data;

    if label != Label::Struct {
        return Err(Error::new(_comma.span, "Expected struct"));
    }

    todo!()
}
