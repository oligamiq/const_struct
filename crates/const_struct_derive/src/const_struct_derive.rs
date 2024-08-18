use crate::ident::get_absolute_ident_path_from_ident;
use convert_case::{Case, Casing as _};
use parse::{Parse, Parser};
use proc_macro2::*;
use quote::{quote, ToTokens as _};
use syn::punctuated::Punctuated;
use syn::*;

/// paramsをwhere節に変換する
pub fn generics_into_where_clause(generics: Generics) -> Generics {
    let mut where_clause = generics.where_clause.unwrap_or_else(|| WhereClause {
        where_token: Default::default(),
        predicates: Default::default(),
    });

    for param in generics.params.clone() {
        if let GenericParam::Const(_) = param {
            continue;
        }
        let param = param.to_token_stream();
        where_clause
            .predicates
            .push(parse2::<WherePredicate>(param).unwrap());
    }

    let params = generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Type(param) => GenericParam::Type(TypeParam {
                bounds: Default::default(),
                ..param.clone()
            }),
            GenericParam::Lifetime(param) => GenericParam::Lifetime(LifetimeParam {
                bounds: Default::default(),
                ..param.clone()
            }),
            GenericParam::Const(_) => param.clone(),
        })
        .collect::<Punctuated<GenericParam, Token![,]>>();

    Generics {
        where_clause: Some(where_clause),
        params,
        ..generics
    }
}

pub fn generate_const_struct_derive(input: DeriveInput) -> Result<TokenStream> {
    let user_attrs = get_const_struct_derive_attr(&input)?;

    let generics: Generics = generics_into_where_clause(input.generics.clone());

    let generics_where_clause = generics
        .where_clause
        .clone()
        .unwrap()
        .predicates
        .iter()
        .map(|pred| match pred {
            WherePredicate::Type(pred) => {
                let ty = &pred.bounds;
                let ty = if ty.iter().any(|ty| match ty {
                    TypeParamBound::Trait(ty) => ty.path.is_ident("Copy"),
                    _ => false,
                }) {
                    ty.clone()
                } else {
                    let mut ty = ty.clone();
                    ty.push(parse_quote!(Copy));
                    ty
                };
                let ty = ty
                    .into_iter()
                    .map(|ty| match ty {
                        TypeParamBound::Trait(ty) => TypeParamBound::Trait(TraitBound {
                            path: user_attrs.get_absolute_path(&ty.path).path(),
                            ..ty
                        }),
                        _ => ty,
                    })
                    .collect::<Punctuated<TypeParamBound, Token![+]>>();
                WherePredicate::Type(PredicateType {
                    bounds: ty,
                    ..pred.clone()
                })
            }
            _ => pred.clone(),
        })
        .collect::<Punctuated<WherePredicate, Token![,]>>();
    let generics = Generics {
        where_clause: Some(WhereClause {
            where_token: Default::default(),
            predicates: generics_where_clause,
        }),
        ..generics
    };

    // println!("generics: {}", generics.to_token_stream());

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

    let mut datatype: Path = parse_quote! { #name };
    let path_segments = datatype.segments.last_mut().unwrap();
    path_segments.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
        colon2_token: None,
        args: generics
            .params
            .iter()
            .map::<GenericArgument, _>(|param| match param {
                GenericParam::Const(ConstParam { ident, .. }) => {
                    GenericArgument::Const(parse_quote! { #ident })
                }
                _ => GenericArgument::Type(parse_quote! { #param }),
            })
            .collect::<Punctuated<_, Token![,]>>(),
        lt_token: Default::default(),
        gt_token: Default::default(),
    });

    let mut new_trait: ItemTrait = parse_quote! {
        #[automatically_derived]
        pub trait #trait_name: ::const_struct::PrimitiveTraits<DATATYPE = #datatype> {
            #(#const_field)*
        }
    };

    // dbg!(&new_trait);

    new_trait.generics = generics.clone();

    // println!("new_trait: {}", new_trait.to_token_stream());

    let trait_name_with_generics = {
        let mut trait_name_with_generics = datatype.clone();
        trait_name_with_generics.segments.first_mut().unwrap().ident = trait_name.clone();
        trait_name_with_generics
    };
    let mut trait_impl: ItemImpl = parse_quote! {
        #[automatically_derived]
        impl<PrimitiveType: ::const_struct::PrimitiveTraits<DATATYPE = #datatype>> #trait_name_with_generics for PrimitiveType {}
    };
    trait_impl.generics.params.extend(generics.params);
    trait_impl.generics.where_clause = generics.where_clause.clone();

    // println!("trait_impl: {}", trait_impl);

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

    pub fn get_absolute_ident_path(&self, ident: &Ident) -> AbsolutePath {
        get_absolute_ident_path_from_ident(ident, self.path_and_ident.clone())
            .unwrap_or(AbsolutePath::new(ident.clone().into()))
    }

    pub fn get_absolute_path(&self, path: &Path) -> AbsolutePath {
        get_absolute_ident_path_from_ident(
            &path.segments.last().unwrap().ident,
            self.path_and_ident.clone(),
        )
        .unwrap_or(AbsolutePath::new(path.clone()))
    }
}

pub fn get_const_struct_derive_attr(input: &DeriveInput) -> Result<ConstStructAttr> {
    let attr = input
        .attrs
        .iter()
        .filter(|attr| attr.path().segments.last().unwrap().ident == "const_struct")
        .collect::<Vec<_>>();

    let is_macro_export = attr.iter().any(|attr| check_macro_export(attr));
    let path_and_ident = attr
        .iter()
        .flat_map(|attr| register_ident_path(attr).unwrap_or_default())
        .collect::<Vec<_>>();

    let attr = ConstStructAttr {
        macro_export: is_macro_export,
        path_and_ident,
    };

    Ok(attr)
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

#[derive(Debug, Clone)]
pub struct PathAndIdent {
    pub ident: Ident,
    pub _token: Token![:],
    pub path: AbsolutePath,
}

#[derive(Debug, Clone)]
pub struct AbsolutePath {
    path: Path,
}

impl AbsolutePath {
    pub fn new(path: Path) -> Self {
        Self { path }
    }

    pub fn path(&self) -> Path {
        let crate_name = std::env::var("CARGO_CRATE_NAME").unwrap();
        if self.path.segments.first().unwrap().ident == crate_name {
            let mut path = self.path.clone();
            path.segments.get_mut(0).unwrap().ident = Ident::new("crate", Span::call_site());
            path.leading_colon = None;
            path
        } else {
            self.path.clone()
        }
    }
}

impl Parse for PathAndIdent {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let _token = input.parse()?;
        let path = input.parse()?;
        Ok(Self {
            ident,
            _token,
            path,
        })
    }
}

impl Parse for AbsolutePath {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let path = input.parse()?;
        Ok(Self { path })
    }
}

pub fn register_ident_path(attr: &Attribute) -> Result<Vec<PathAndIdent>> {
    let attr_token = match get_token(attr) {
        Some(token) => token,
        None => {
            return Err(Error::new_spanned(
                attr,
                "Expected #[const_struct(ident: mod::ident)]",
            ))
        }
    };
    let attr_args = syn::punctuated::Punctuated::<PathAndIdent, Token![,]>::parse_terminated
        .parse2(attr_token)?;

    // dbg!(&attr_args);

    Ok(attr_args.into_iter().collect())
}
