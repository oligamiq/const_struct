use proc_macro2::{Spacing, TokenStream};
use quote::{quote, TokenStreamExt as _};
use syn::Ident;

pub fn add_at_mark(ident: Ident) -> TokenStream {
    let mut tokens = TokenStream::new();

    // `@` をトークンとして追加
    tokens.append(proc_macro2::Punct::new('@', Spacing::Joint));

    // `ident` をトークンとして追加
    tokens.extend(quote! { #ident });

    tokens
}
