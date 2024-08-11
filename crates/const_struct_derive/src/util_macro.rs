use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenTree};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Token,
};

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

pub fn match_underscore(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TtAndTt);

    if input.input.to_string() == "_" {
        let is_underscore = input.tt_is_underscore;
        TokenStream::from(quote! { #is_underscore })
    } else {
        let input = input.input;
        TokenStream::from(quote! { #input })
    }
}
