use crate::ident::get_absolute_ident_path_from_ident;
use crate::ident::AbsolutePathOrType;
use crate::parse_value::AdditionData;
use crate::util::add_at_mark;
use crate::util::add_dollar_mark;
use crate::util::check_meta_path;
use crate::util::gen_get_const_generics_inner;
use crate::util::item_fn_with_meta;
use crate::util_macro::ConstOrType;
use convert_case::Case;
use convert_case::Casing;
use parse::discouraged::Speculative as _;
use parse::{Parse, Parser};
use proc_macro2::*;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
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

    let vis = &input.vis;

    let generics: Generics = generics_into_where_clause(input.generics.clone());

    let generics_where_clause_fn = |with_copy: bool| {
        generics
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
                        if with_copy {
                            ty.push(parse_quote!(Copy));
                        }
                        ty
                    };
                    let ty = ty
                        .into_iter()
                        .map(|ty| match ty {
                            TypeParamBound::Trait(ty) => TypeParamBound::Trait(TraitBound {
                                path: match user_attrs.get_absolute_path(&ty.path) {
                                    AbsolutePathOrType::Path(path) => path.path(),
                                    AbsolutePathOrType::Type(_) => panic!("Type is not allowed"),
                                },
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
            .collect::<Punctuated<WherePredicate, Token![,]>>()
    };

    let generics_where_clause = generics_where_clause_fn(false);
    let generics_where_clause_with_copy = generics_where_clause_fn(true);

    let generics = Generics {
        where_clause: Some(WhereClause {
            where_token: Default::default(),
            predicates: generics_where_clause,
        }),
        ..generics
    };
    let generics_with_copy = Generics {
        where_clause: Some(WhereClause {
            where_token: Default::default(),
            predicates: generics_where_clause_with_copy,
        }),
        ..generics.clone()
    };

    let name = &input.ident;
    let datatype = {
        let mut datatype: Path = parse_quote! { #name };
        let path_segments = datatype.segments.last_mut().unwrap();
        path_segments.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: None,
            args: generics
                .params
                .iter()
                .map::<GenericArgument, _>(|param| match param {
                    GenericParam::Const(ConstParam { ident, .. }) => {
                        GenericArgument::Const(parse_quote! { { #ident } })
                    }
                    _ => GenericArgument::Type(parse_quote! { #param }),
                })
                .collect::<Punctuated<_, Token![,]>>(),
            lt_token: Default::default(),
            gt_token: Default::default(),
        });
        datatype
    };

    let keep_type_impls = generics
        .params
        .iter()
        .enumerate()
        .filter_map(|(num, param)| match param {
            GenericParam::Const(param) => {
                let ty = &param.ty;
                let keeptype = user_attrs
                    .get_absolute_path_path(&parse_quote! { ::const_struct::keeptype::KeepType });
                let mut keep_type_impl: ItemImpl = parse_quote! {
                    #[automatically_derived]
                    #[doc(hidden)]
                    impl #keeptype<#num> for #datatype {
                        type Type = #ty;
                    }
                };
                keep_type_impl.generics = generics.clone();

                Some(keep_type_impl)
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    // println!("generics: {}", generics.to_token_stream());

    // dbg!(&input.data);

    let fields = match &input.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Unnamed fields are not allowed yet",
                ))
            }
            Fields::Unit => &Punctuated::new(),
        },
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

    let primitive_traits_path =
        user_attrs.get_absolute_path_path(&parse_quote! { ::const_struct::PrimitiveTraits });

    let const_field = field_names
        .zip(field_types)
        .map(|(field, ty)| {
            let field = field.as_ref().unwrap();
            let upper_field = get_upper_filed_name(field);
            quote! {
                const #upper_field: #ty = <Self as #primitive_traits_path>::__DATA.#field;
            }
        })
        .collect::<Vec<_>>();

    let mut new_trait_impl: ItemTrait = parse_quote! {
        #[automatically_derived]
        pub trait #trait_name: #primitive_traits_path<DATATYPE = #datatype> {
            #(#const_field)*
        }
    };

    // dbg!(&new_trait_impl);

    new_trait_impl.generics = generics_with_copy.clone();

    // println!("new_trait_impl: {}", new_trait_impl.to_token_stream());

    let trait_name_with_generics = {
        let mut trait_name_with_generics = datatype.clone();
        trait_name_with_generics.segments.first_mut().unwrap().ident = trait_name.clone();
        trait_name_with_generics
    };
    let mut trait_impl: ItemImpl = parse_quote! {
        #[automatically_derived]
        impl<PrimitiveType: #primitive_traits_path<DATATYPE = #datatype>> #trait_name_with_generics for PrimitiveType {}
    };
    trait_impl
        .generics
        .params
        .extend(generics_with_copy.params.clone());
    trait_impl.generics.where_clause = generics_with_copy.where_clause.clone();

    // println!("### 1 ###");

    let name_with_get_generics_data = add_at_mark(format_ident!("{}GetGenericsData", name));
    let addition_data = &user_attrs.addition_data;
    let mut const_fn: ItemFn = parse_quote!(
        const fn get_const_generics(_: #datatype) {}
    );
    const_fn.vis = vis.clone();
    const_fn.sig.generics = generics_with_copy.clone();
    let const_fn = item_fn_with_meta(const_fn);

    let generics_snake = generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Const(ConstParam { ident, .. }) => (ident, ConstOrType::Const),
            GenericParam::Type(TypeParam { ident, .. }) => (ident, ConstOrType::Type),
            GenericParam::Lifetime(LifetimeParam { .. }) => {
                eprintln!("error: lifetime is not allowed");
                unreachable!()
            }
        })
        .map(|(ident, const_or_type)| {
            (
                format_ident!(
                    "{}",
                    ident
                        .to_string()
                        .from_case(Case::UpperCamel)
                        .to_case(Case::Snake)
                ),
                const_or_type,
            )
        })
        .collect::<Vec<_>>();

    let mut macro_args = generics_snake
        .iter()
        .map(|(ident, const_or_type)| {
            let ident_with_dollar = add_dollar_mark(ident.clone());
            match const_or_type {
                ConstOrType::Const => quote! { #ident_with_dollar: tt },
                ConstOrType::Type => quote! { #ident_with_dollar: path },
            }
        })
        .collect::<Punctuated<_, Token![,]>>();
    macro_args.push(quote! { $value: expr });

    let hash_bridge = user_attrs
        .get_absolute_path_meta_path(&parse_quote! { ::const_struct::primitive::HashBridge });
    let hash_bridge_bridge = user_attrs
        .get_absolute_path_meta_path(&parse_quote! { ::const_struct::primitive::HashBridgeBridge });
    let str_hash = user_attrs
        .get_absolute_path_meta_path(&parse_quote! { ::const_struct::primitive::str_hash });
    let match_underscore_path =
        user_attrs.get_absolute_path_meta_path(&parse_quote! { ::const_struct::match_underscore });
    let match_end_with_path =
        user_attrs.get_absolute_path_meta_path(&parse_quote! { ::const_struct::match_end_with });

    let gen_args = generics_snake
        .iter()
        .enumerate()
        .map(|(num, (ident, const_or_type))| {
            let ident_with_dollar = add_dollar_mark(ident.clone());

            match const_or_type {
                ConstOrType::Const => {
                    let get_const_generics_fn_seed =
                        gen_get_const_generics_inner(const_fn.clone(), num).unwrap();
                    let fn_ident = get_const_generics_fn_seed.sig.ident.clone();
                    quote! { {
                        #match_underscore_path!(#ident_with_dollar, {
                            #get_const_generics_fn_seed

                            #fn_ident($value)
                        })
                    }}
                }
                ConstOrType::Type => {
                    quote! { #ident_with_dollar }
                }
            }
        })
        .collect::<Punctuated<TokenStream, Token![,]>>();

    // println!("gen_args: {}", gen_args.to_token_stream());

    let mut macro_matches = Vec::new();
    macro_matches.push(quote! {
        (#name_with_get_generics_data, $macro_path: path, $($arg:tt)*) => {
            {
                $macro_path!(
                    @AdditionData(
                        #addition_data
                    ),
                    #name_with_get_generics_data(
                        struct,
                        #const_fn
                    ),
                    $($arg)*
                )
            }
        };
    });

    let generate_macro_match = |gen_args: &Punctuated<TokenStream, Token![,]>| {
        quote! {
            #hash_bridge<{
                const NAME_HASH: u64 = #str_hash(stringify!($value));

                type T = #name<#gen_args>;

                impl #hash_bridge_bridge<NAME_HASH, {#str_hash(file!())}, {column!()}, {line!()}> for T {
                    type DATATYPE = T;
                    const DATA: Self::DATATYPE = {
                        $value
                    };
                }

                NAME_HASH
            }, {
                #str_hash(file!())
            }, {
                column!()
            }, {
                line!()
            },
            #name<#gen_args>
            >
        }
    };

    macro_matches.push({
        let ty = generate_macro_match(&gen_args);
        quote! {
            (#macro_args) => {
                #ty
            };
        }
    });

    if generics_snake
        .iter()
        .filter(|(_, const_or_type)| *const_or_type == ConstOrType::Const)
        .count()
        != 0
    {
        let gen_args = generics_snake
            .iter()
            .enumerate()
            .map(|(num, (ident, const_or_type))| match const_or_type {
                ConstOrType::Const => {
                    let get_const_generics_fn_seed =
                        gen_get_const_generics_inner(const_fn.clone(), num).unwrap();
                    let fn_ident = get_const_generics_fn_seed.sig.ident.clone();
                    quote! { {
                        #get_const_generics_fn_seed

                        #fn_ident($value)
                    } }
                }
                ConstOrType::Type => {
                    let ident_with_dollar = add_dollar_mark(ident.clone());
                    quote! { #ident_with_dollar }
                }
            })
            .collect::<Punctuated<TokenStream, Token![,]>>();

        let ty = generate_macro_match(&gen_args);
        macro_matches.push(
            if generics_snake
                .iter()
                .filter(|(_, const_or_type)| *const_or_type == ConstOrType::Type)
                .count()
                == 0
            {
                quote! {
                    ($value: expr) => {
                        #match_end_with_path!($value, #ty)
                    };
                }
            } else {
                let mut macro_args = generics_snake
                    .iter()
                    .filter_map(|(ident, const_or_type)| {
                        let ident_with_dollar = add_dollar_mark(ident.clone());
                        match const_or_type {
                            ConstOrType::Const => None,
                            ConstOrType::Type => Some(quote! { #ident_with_dollar: path }),
                        }
                    })
                    .collect::<Punctuated<_, Token![,]>>();
                macro_args.push(quote! { $value: expr });

                quote! {
                    (#macro_args) => {
                        #ty
                    };
                }
            },
        );
    }
    if generics_snake
        .iter()
        .filter(|(_, const_or_type)| *const_or_type == ConstOrType::Type)
        .count()
        != 0
    {
        let marker_path =
            user_attrs.get_absolute_path_path(&parse_quote! { ::const_struct::keeptype::Marker });
        macro_matches.push(quote! {
            ($value: expr) => {
                #match_end_with_path!($value, #marker_path<{
                    compile_error!("Expected a ???Ty. If not ???Ty, you can only omission const generics")
                }>)
            };
        });
    }

    let macro_export = quote! {
        macro_rules! #name {
            #(#macro_matches)*
        }
    };
    let macro_export = if user_attrs.macro_export {
        let name_with_underscore = format_ident!("_{name}");
        let name_module = format_ident!("__{}", name.to_string().to_case(Case::Snake));
        let use_: ItemUse = parse_quote!(pub(crate) use #name as #name_with_underscore;);
        let use_outer: ItemUse =
            parse_quote!(pub(crate) use #name_module::#name_with_underscore as #name;);
        quote! {
            #[doc(hidden)]
            pub mod #name_module {
                #[macro_export]
                #[allow(unused_macros)]
                #macro_export

                #[doc(hidden)]
                #[allow(unused_imports)]
                #use_
            }
            #[doc(hidden)]
            #[allow(unused_imports)]
            #use_outer
        }
    } else {
        quote! {
            #[allow(unused_macros)]
            #macro_export
        }
    };

    // println!("macro_export: {}", macro_export.to_token_stream());

    Ok(quote! {
        #(#keep_type_impls)*
        #new_trait_impl
        #trait_impl
        #macro_export
    })
}

#[derive(Debug, Default)]
pub struct ConstStructAttr {
    macro_export: bool,
    addition_data: AdditionData,
}

impl ConstStructAttr {
    pub fn get_absolute_path(&self, path: &Path) -> AbsolutePathOrType {
        Self::get_absolute_path_inner(path, &self.addition_data.data)
    }

    pub fn get_absolute_path_path(&self, path: &Path) -> Path {
        match self.get_absolute_path(path) {
            AbsolutePathOrType::Path(path) => path.path(),
            AbsolutePathOrType::Type(_) => {
                eprintln!("error: expected path, found type");
                unreachable!()
            }
        }
    }

    pub fn get_absolute_path_meta_path(&self, path: &Path) -> TokenStream {
        check_meta_path(&self.get_absolute_path_path(path))
    }

    pub fn get_absolute_path_inner(
        path: &Path,
        path_and_ident: &Vec<PathAndIdent>,
    ) -> AbsolutePathOrType {
        get_absolute_ident_path_from_ident(path, path_and_ident)
            .unwrap_or(AbsolutePathOrType::Path(AbsolutePath::new(path.clone())))
    }
}

pub fn get_const_struct_derive_attr(input: &DeriveInput) -> Result<ConstStructAttr> {
    let attr = input
        .attrs
        .iter()
        .filter(|attr| {
            let path = attr.path();
            let path = path.to_token_stream().to_string();
            path == "const_struct"
                || path == "const_struct :: const_struct"
                || path == ":: const_struct :: const_struct"
        })
        .collect::<Vec<_>>();

    let is_macro_export = attr.iter().any(|attr| check_macro_export(attr));
    let path_and_ident = attr
        .iter()
        .flat_map(|attr| register_ident_path(attr).unwrap_or_default())
        .collect::<Vec<_>>();

    let addition_data = AdditionData {
        data: path_and_ident,
    };

    let attr = ConstStructAttr {
        macro_export: is_macro_export,
        addition_data,
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
    pub ident: Path,
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
        let fork = input.fork();
        let ident = fork.parse()?;
        let _token = fork.parse()?;
        let path = fork.parse()?;
        input.advance_to(&fork);
        Ok(Self {
            ident,
            _token,
            path,
        })
    }
}

impl ToTokens for PathAndIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let path = &self.path;
        tokens.extend(quote! { #ident: #path });
    }
}

impl Parse for AbsolutePath {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let path = input.parse()?;
        Ok(Self { path })
    }
}

impl ToTokens for AbsolutePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = &self.path;
        tokens.extend(quote! { #path });
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
