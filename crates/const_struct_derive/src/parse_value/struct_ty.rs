use crate::util_macro::GenericsData;

pub fn parse_value_struct_ty(struct_data: GenericsData) -> Type {
    let GenericsData {
        _at,
        ident,
        _paren_token,
        const_fn,
        _comma,
        macros,
        label,
        _comma2,
    } = struct_data;

    todo!()
}
