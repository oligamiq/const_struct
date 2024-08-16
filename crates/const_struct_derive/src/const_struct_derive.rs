use std::f32::consts::E;

use convert_case::{Case, Casing as _};
use parse::{Parse, Parser};
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

#[derive(Debug)]
pub struct ConstStructAttr {
    macro_export: bool,
    path_and_ident: Vec<PathAndIdent>,
}

impl ConstStructAttr {
    pub fn default() -> Self {
        Self {
            macro_export: false,
            path_and_ident: Vec::new(),
        }
    }
}

pub fn get_const_struct_derive_attr(input: &DeriveInput) -> Result<ConstStructAttr> {
    let attr = input
        .attrs
        .iter()
        .filter(|attr| attr.path().segments.last().unwrap().ident == "const_struct")
        .collect::<Vec<_>>();

    let is_macro_export = attr.iter().any(|attr| check_macro_export(attr));
    let path_and_ident = attr.iter().flat_map(|attr| register_ident_path(attr).unwrap_or_default()).collect::<Vec<_>>();

    let attr = ConstStructAttr {
        macro_export: is_macro_export,
        path_and_ident,
    };

    dbg!(&attr);

    // TODO!()
    Ok(ConstStructAttr::default())
}

pub fn get_token(attr: &Attribute) -> Option<TokenStream> {
    let attr_token = match attr.meta {
        Meta::List(ref list) => list.tokens.clone(),
        _ => return None,
    };
    Some(attr_token)
}

pub fn check_macro_export(attr: &Attribute) -> bool {
    if let Some(attr_token) = get_token(attr) {
        let parse_macro_export = parse2::<Ident>(attr_token);
        match parse_macro_export {
            Ok(ident) => ident == "macro_export",
            Err(_) => false,
        }
    } else {
        false
    }
}

#[derive(Debug)]
pub struct PathAndIdent {
    pub ident: Ident,
    pub _token: Token![:],
    pub path: DollarPath,
}

#[derive(Debug)]
pub struct DollarPath {
    pub meta_dollar: Option<Token![$]>,
    pub path: Path,
}

impl Parse for PathAndIdent {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let _token = input.parse()?;
        let path = input.parse()?;
        Ok(Self { ident, _token, path })
    }
}

impl Parse for DollarPath {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let meta_dollar = input.parse()?;
        let path = input.parse()?;
        Ok(Self { meta_dollar, path })
    }
}

pub fn register_ident_path(attr: &Attribute) -> Result<Vec<PathAndIdent>> {
    let attr_token = match get_token(attr) {
        Some(token) => token,
        None => return Err(Error::new_spanned(attr, "Expected #[const_struct(ident: mod::ident)]")),
    };
    let attr_args = syn::punctuated::Punctuated::<PathAndIdent, Token![,]>::parse_terminated
        .parse2(attr_token)?;

    dbg!(&attr_args);

    Ok(attr_args.into_iter().collect())
}
