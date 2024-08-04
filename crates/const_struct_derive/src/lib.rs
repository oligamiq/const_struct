// https://astexplorer.net/

use convert_case::{Case, Casing as _};
use proc_macro::TokenStream as RawTokenStream;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rewriter::Rewriter as _;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input, parse_quote, parse_str,
    token::Token,
    Attribute, Data, DataStruct, DeriveInput, Expr, ExprMacro, Fields, FieldsNamed, Ident,
    ItemConst, ItemFn, Meta, MetaList, PatMacro, Token, TypeParam,
};

mod rewriter;

#[proc_macro_derive(ConstStruct)]
pub fn const_struct_derive(input: RawTokenStream) -> RawTokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = generate_const_struct_derive(input);
    match output {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_const_struct_derive(input: DeriveInput) -> Result<TokenStream, syn::Error> {
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

    return Ok(quote! {
        #new_trait
        #trait_impl
    });
}

#[proc_macro_attribute]
pub fn const_struct(_attr: RawTokenStream, item: RawTokenStream) -> RawTokenStream {
    let input = parse_macro_input!(item as ItemConst);
    let output = generate_const_struct(input);
    match output {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_const_struct(input: ItemConst) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;
    let ty = &input.ty;

    // dbg!(&input);

    let ty_name = {
        let name_upper_snake = name.to_string();
        let name_pascal = name_upper_snake
            .from_case(Case::UpperSnake)
            .to_case(Case::Pascal);
        quote::format_ident!("{}Ty", name_pascal)
    };

    let struct_define = quote! {
        pub struct #ty_name;
    };

    let struct_impl = quote! {
        impl ::const_struct::ConstStructTraits<#ty> for #ty_name {
            const __DATA: #ty = #name;
        }
    };

    Ok(quote! {
        #input
        #struct_define
        #struct_impl
    })
}

#[proc_macro_attribute]
pub fn const_compat(attr: RawTokenStream, item: RawTokenStream) -> RawTokenStream {
    match syn::parse::<ItemFn>(item.clone()) {
        Ok(input) => {
            let output = generate_const_compat_fn(input, attr.into());
            match output {
                Ok(output) => output.into(),
                Err(err) => err.to_compile_error().into(),
            }
        }
        Err(err) => match syn::parse::<Expr>(item) {
            Ok(input) => {
                let output = generate_const_compat_expr(input, attr.into());
                match output {
                    Ok(output) => output.into(),
                    Err(err) => err.to_compile_error().into(),
                }
            }
            Err(_) => err.to_compile_error().into(),
        },
    }
    // let output = generate_const_compat(input);
    // match output {
    //     Ok(output) => output.into(),
    //     Err(err) => err.to_compile_error().into(),
    // }
}

#[derive(Debug)]
enum ConstCompatAttr {
    Ident(Ident),
    Attribute(Attribute),
}

impl Parse for ConstCompatAttr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        match input.parse::<syn::Ident>() {
            Ok(meta) => Ok(ConstCompatAttr::Ident(meta)),
            Err(_) => {
                let attrs = input.call(Attribute::parse_outer)?;
                if attrs.len() != 1 {
                    return Err(syn::Error::new_spanned(
                        input.cursor().token_stream(),
                        "Expected exactly one attribute",
                    ));
                }
                Ok(ConstCompatAttr::Attribute(attrs.first().unwrap().clone()))
            }
        }
    }
}

fn generate_const_compat_fn(input: ItemFn, attr: TokenStream) -> Result<TokenStream, syn::Error> {
    let attr_args = syn::punctuated::Punctuated::<ConstCompatAttr, Token![,]>::parse_terminated
        .parse2(attr.clone())?;
    let mut iter = attr_args.into_iter();
    let root_ident = match iter.next() {
        Some(ConstCompatAttr::Ident(ident)) => ident,
        _ => {
            return Err(syn::Error::new_spanned(
                attr,
                "First attribute must be an identifier",
            ))
        }
    };
    let root_cfg = match iter.next() {
        Some(ConstCompatAttr::Attribute(cfg)) => cfg,
        _ => {
            return Err(syn::Error::new_spanned(
                attr,
                "Second attribute must be an attribute",
            ))
        }
    };
    match iter.next() {
        None => {}
        _ => return Err(syn::Error::new_spanned(attr, "Too many attributes")),
    }

    // dbg!(&root_cfg);

    let new_input = input.clone();
    let (new_input, ty) = match new_input.sig.inputs.iter().position(|arg| {
        if let syn::FnArg::Typed(pat) = arg {
            if let syn::Pat::Ident(ident) = &*pat.pat {
                ident.ident == root_ident
            } else {
                false
            }
        } else {
            false
        }
    }) {
        Some(i) => {
            let arg = &new_input.sig.inputs[i];
            let pat = match arg {
                syn::FnArg::Typed(pat) => pat,
                _ => return Err(syn::Error::new_spanned(arg, "Expected a typed argument")),
            };
            let ty = &pat.ty;
            let new_inputs = new_input
                .sig
                .inputs
                .clone()
                .into_iter()
                .enumerate()
                .filter_map(|(j, arg)| if i != j { Some(arg) } else { None })
                .collect::<syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>>();
            let new_input = syn::ItemFn {
                sig: syn::Signature {
                    inputs: new_inputs,
                    ..new_input.sig.clone()
                },
                ..new_input
            };
            (new_input, ty)
        }
        None => {
            return Err(syn::Error::new_spanned(
                new_input.sig.inputs,
                "No argument with the given name",
            ))
        }
    };

    let new_generic = new_input.sig.generics.clone();
    let new_generic_name = root_ident
        .to_string()
        .from_case(Case::Snake)
        .to_case(Case::UpperSnake);
    let generics_name = format!("{}: {}Ty", new_generic_name, ty.to_token_stream());
    let generics_name = parse_str::<TypeParam>(&generics_name).unwrap();
    let new_generics = {
        let mut new_generics = new_generic.clone();
        new_generics
            .params
            .push(syn::GenericParam::Type(generics_name));
        new_generics
    };
    // 関数名を変更
    let new_name = format!("{}_const", root_ident);
    let new_input = syn::ItemFn {
        sig: syn::Signature {
            ident: syn::Ident::new(&new_name, new_input.sig.ident.span()),
            generics: new_generics,
            ..new_input.sig
        },
        ..new_input
    };

    let not_root_cfg = root_cfg.clone();
    let meta = not_root_cfg.meta;
    let meta = match meta {
        Meta::List(list) => {
            let old_token = list.tokens.clone();
            let new_token = quote! { not(#old_token) };
            let new_meta = Meta::List(syn::MetaList {
                tokens: new_token,
                ..list
            });
            new_meta
        }
        _ => return Err(syn::Error::new_spanned(meta, "Expected a list")),
    };
    let not_root_cfg = Attribute {
        meta,
        ..not_root_cfg
    };

    let new_input = new_input;
    let new_input = new_input.rewrite(|path_segment| {
        if path_segment.ident == root_ident {
            let __data: syn::PathSegment = parse_str("__DATA").unwrap();
            let new_generic_name: syn::PathSegment = parse_str(&new_generic_name).unwrap();
            vec![new_generic_name, __data].into_iter().collect()
        } else {
            vec![path_segment].into_iter().collect()
        }
    }, |ident| {
        ident == root_ident
    });
    let new_input = ItemFn {
        sig: syn::Signature {

            ..new_input.sig
        },
        ..new_input
    };

    // dbg!(ty);

    let output = quote! {
        #root_cfg
        #input
        #not_root_cfg
        #new_input
    };
    Ok(output)
}

fn generate_const_compat_expr(input: Expr, attr: TokenStream) -> Result<TokenStream, syn::Error> {
    let cfg = match syn::parse::<syn::MetaList>(attr.into()) {
        Ok(cfg) => cfg,
        Err(err) => return Err(err),
    };

    let output = quote! {
        #input
    };
    Ok(output)
}
