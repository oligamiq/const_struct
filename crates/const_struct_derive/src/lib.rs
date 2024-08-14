// https://astexplorer.net/

use const_compat::{generate_const_compat_fn, generate_const_struct};
use const_struct_derive::generate_const_struct_derive;
use darling::ast::NestedMeta;
use proc_macro::TokenStream as RawTokenStream;
use quote::ToTokens as _;
use syn::{parse, parse_macro_input, parse_quote, Attribute, DeriveInput, ItemConst, ItemFn, ItemStruct};

mod const_compat;
mod const_struct_derive;
mod rewriter;
mod util_macro;

#[proc_macro_derive(ConstStruct)]
pub fn const_struct_derive(input: RawTokenStream) -> RawTokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dbg!(&input);
    let output = generate_const_struct_derive(input);
    match output {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn const_struct(attr: RawTokenStream, item: RawTokenStream) -> RawTokenStream {
    let output = match parse::<ItemConst>(item.clone()) {
        Ok(input) => generate_const_struct(input),
        Err(err) => match parse::<ItemStruct>(item.clone()) {
            Ok(st) => {
                // dbg!(&st);
                if st.attrs.is_empty() {
                    return item;
                } else {
                    let mut st = st;
                    let old_attr = &mut st.attrs;
                    let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
                        Ok(v) => v,
                        Err(e) => return e.to_compile_error().into(),
                    };
                    let self_attr: Attribute = parse_quote! {
                        #[const_struct(#(#attr_args)*)]
                    };
                    old_attr.push(self_attr);
                    Ok(st.to_token_stream())
                }
            },
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
