use convert_case::{Case, Casing as _};
use proc_macro2::*;
use quote::quote;
use syn::*;

pub fn generate_const_struct_derive(input: DeriveInput) -> Result<TokenStream> {
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
                const #upper_field: #ty = Self::__DATA.#field;
            }
        })
        .collect::<Vec<_>>();

    let new_trait = quote! {
        pub trait #trait_name: ::const_struct::ConstStructTraits<#name> {
            #(#const_field)*
        }
    };

    let trait_impl = quote! {
        impl<T: ::const_struct::ConstStructTraits<#name>> #trait_name for T {}
    };

    Ok(quote! {
        #new_trait
        #trait_impl
    })
}
