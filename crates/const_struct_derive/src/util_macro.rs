use parse::{Parse, ParseStream, Parser as _};
use proc_macro::TokenStream as TokenStreamRaw;
use proc_macro2::{TokenStream, TokenTree};
use punctuated::Punctuated;
use quote::quote;
use syn::*;

pub struct TtAndTt {
    input: TokenTree,
    _comma: Token![,],
    tt_is_underscore: TokenTree,
}

impl Parse for TtAndTt {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            input: input.parse()?,
            _comma: input.parse()?,
            tt_is_underscore: input.parse()?,
        })
    }
}

pub fn match_underscore(input: TokenStreamRaw) -> TokenStreamRaw {
    let input = parse_macro_input!(input as TtAndTt);

    if input.input.to_string() == "_" {
        let is_underscore = input.tt_is_underscore;
        quote! { #is_underscore }
    } else {
        let input = input.input;
        quote! { #input }
    }
    .into()
}

pub fn expand_call_fn_with_generics(input: TokenStream) -> Result<TokenStream> {
    let input = parse_macro_input!(input as Punctuated<FnArg, Token![,]>);

    // dbg!(&input);

    let mut token_stream = TokenStream::new();

    let mut iter = input.into_iter();
    match iter.next() {
        Some(tt) => {
            let tt = tt.stmts;
            token_stream.extend(quote! { #(#tt)* });
        }
        _ => {}
    }
    for tt in iter {
        let tt = tt.stmts;
        token_stream.extend(quote! {, #(#tt)* });
    }

    println!("token_stream: {}", token_stream.to_string());

    Ok(token_stream)
}
