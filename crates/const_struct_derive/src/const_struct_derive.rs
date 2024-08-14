use convert_case::{Case, Casing as _};
use darling::ast::NestedMeta;
use proc_macro2::*;
use quote::quote;
use syn::*;

pub fn generate_const_struct_derive(input: DeriveInput) -> Result<TokenStream> {
    let user_attrs = get_const_struct_derive_attr(&input)?;

    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => return Err(syn::Error::new_spanned(input, "Must be struct type")),
    };

    let field_names = fields.iter().map(|field| &field.ident);
    let field_types = fields.iter().map(|field| &field.ty);

    let trait_name = quote::format_ident!("{}Ty", name);

    // to UPPER_SNAKE_CASE
    fn get_upper_filed_name(field: &syn::Ident) -> syn::Ident {
        let field_name_snake = field.to_string();
        let field_name_upper_snake = field_name_snake
            .from_case(Case::Snake)
            .to_case(Case::UpperSnake);
        quote::format_ident!("{}", field_name_upper_snake)
    }

    let const_field = field_names
        .zip(field_types)
        .map(|(field, ty)| {
            let field = field.as_ref().unwrap();
            let upper_field = get_upper_filed_name(field);
            quote! {
                const #upper_field: #ty = <Self as ::const_struct::PrimitiveTraits>::__DATA.#field;
            }
        })
        .collect::<Vec<_>>();

    let new_trait = quote! {
        #[automatically_derived]
        pub trait #trait_name: ::const_struct::PrimitiveTraits<DATATYPE = #name> {
            #(#const_field)*
        }
    };

    let trait_impl = quote! {
        #[automatically_derived]
        impl<PrimitiveType: ::const_struct::PrimitiveTraits<DATATYPE = #name>> #trait_name for PrimitiveType {}
    };

    Ok(quote! {
        #new_trait
        #trait_impl
    })
}

pub fn get_const_struct_derive_attr(input: &DeriveInput) -> Result<Option<Vec<NestedMeta>>> {
    let attr = match input.attrs.iter().find(|attr| {
        attr.path().segments.last().unwrap().ident == "const_struct"
    }) {
        Some(attr) => attr,
        None => return Ok(None),
    };
    let attr_args = match NestedMeta::parse_meta_list(match attr.meta {
        Meta::List(ref list) => {
            list.tokens.clone()
        },
        _ => return Err(Error::new_spanned(attr, "Expected #[const_struct(...)]")),
    }) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(Some(attr_args))
}
