// https://astexplorer.net/

use const_compat::{generate_const_compat_fn, generate_const_struct};
use const_struct_derive::generate_const_struct_derive;
use proc_macro::TokenStream as RawTokenStream;
use syn::{parse_macro_input, DeriveInput, ItemConst, ItemFn};

mod const_compat;
mod const_struct_derive;
mod rewriter;
mod util_macro;

#[proc_macro_derive(ConstStruct)]
pub fn const_struct_derive(input: RawTokenStream) -> RawTokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = generate_const_struct_derive(input);
    match output {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
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

#[proc_macro]
pub fn match_underscore(input: RawTokenStream) -> RawTokenStream {
    let output = util_macro::match_underscore(input);
    output.into()
}
