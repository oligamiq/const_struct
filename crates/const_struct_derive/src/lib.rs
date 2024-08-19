// https://astexplorer.net/

use const_compat::{generate_const_compat_fn, generate_const_struct};
use const_struct_derive::generate_const_struct_derive;
use proc_macro::TokenStream as RawTokenStream;
use proc_macro2::TokenStream;
use quote::ToTokens as _;
use syn::{
    parse, parse_macro_input, parse_quote, Attribute, DeriveInput, ItemConst, ItemFn, ItemStruct,
    Meta, MetaList,
};

mod const_compat;
mod const_struct_derive;
mod ident;
mod rewriter;
mod util;
mod util_macro;

#[proc_macro_derive(ConstStruct)]
pub fn const_struct_derive(input: RawTokenStream) -> RawTokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // dbg!(&input);
    let output = generate_const_struct_derive(input);
    match output {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn const_struct(attr: RawTokenStream, item: RawTokenStream) -> RawTokenStream {
    fn check_derive_attr(attr: &Attribute) -> bool {
        attr.path().is_ident("derive")
            || match attr.meta {
                Meta::List(MetaList { ref tokens, .. }) => {
                    tokens.clone().into_iter().any(|token| match token {
                        proc_macro2::TokenTree::Ident(ident) => ident == "ConstStruct",
                        _ => false,
                    })
                }
                _ => false,
            }
    }

    let output = match parse::<ItemConst>(item.clone()) {
        Ok(input) => generate_const_struct(input),
        Err(err) => match parse::<ItemStruct>(item.clone()) {
            Ok(st) => {
                // dbg!(&st);
                let index = st.attrs.iter().position(check_derive_attr);
                if let Some(index) = index {
                    let mut st = st;
                    let old_attr = &mut st.attrs;
                    let attr: TokenStream = attr.into();
                    let self_attr: Attribute = parse_quote! {
                        #[const_struct(#attr)]
                    };
                    old_attr.insert(index + 1, self_attr);
                    Ok(st.to_token_stream())
                } else {
                    return item;
                }
            }
            Err(_) => Err(err),
        },
    };
    match output {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
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
        // Err(err) => match syn::parse::<Expr>(item) {
        //     Ok(input) => {
        //         let output = generate_const_compat_expr(input, attr.into());
        //         match output {
        //             Ok(output) => output.into(),
        //             Err(err) => err.to_compile_error().into(),
        //         }
        //     }
        Err(err) => err.to_compile_error().into(),
        // },
    }
    // let output = generate_const_compat(input);
    // match output {
    //     Ok(output) => output.into(),
    //     Err(err) => err.to_compile_error().into(),
    // }
}

/// macro expansion ignores token `,` and any following rustc
#[proc_macro]
pub fn call_with_generics(input: RawTokenStream) -> RawTokenStream {
    let output = util_macro::expand_call_fn_with_generics(input.into());
    match output {
        Ok(output) => output.into(),

        Err(err) => err.to_compile_error().into(),
    }
}
