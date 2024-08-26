use proc_macro2::{Spacing, TokenStream};
use punctuated::Punctuated;
use quote::{quote, ToTokens as _, TokenStreamExt as _};
use syn::*;

pub fn add_at_mark(ident: Ident) -> TokenStream {
    let mut tokens = TokenStream::new();

    // `@` をトークンとして追加
    tokens.append(proc_macro2::Punct::new('@', Spacing::Joint));

    // `ident` をトークンとして追加
    tokens.extend(quote! { #ident });

    tokens
}

pub fn add_dollar_mark_inner(stream: TokenStream) -> TokenStream {
    let mut tokens = TokenStream::new();

    // `$` をトークンとして追加
    tokens.append(proc_macro2::Punct::new('$', Spacing::Joint));

    // `stream` をトークンとして追加
    tokens.extend(stream);

    tokens
}

pub fn add_dollar_mark(ident: Ident) -> TokenStream {
    add_dollar_mark_inner(quote! { #ident })
}

pub fn check_meta_path(path: &Path) -> TokenStream {
    if let (
        None,
        Some(PathSegment {
            ident,
            arguments: PathArguments::None,
        }),
    ) = (path.leading_colon, path.segments.first())
    {
        if ident == "crate" {
            add_dollar_mark_inner(path.to_token_stream())
        } else {
            path.to_token_stream()
        }
    } else {
        path.to_token_stream()
    }
}

pub fn item_fn_with_meta(mut item_fn: ItemFn) -> ItemFn {
    let predicates = &mut item_fn
        .sig
        .generics
        .where_clause
        .as_mut()
        .unwrap()
        .predicates;
    *predicates = predicates
        .iter()
        .cloned()
        .map(|pred| {
            match pred {
                WherePredicate::Type(PredicateType {
                    bounded_ty,
                    bounds,
                    lifetimes,
                    colon_token,
                }) => {
                    let new_bounds = bounds
                        .iter()
                        .cloned()
                        .map(|bound| {
                            if let TypeParamBound::Trait(TraitBound {
                                paren_token,
                                modifier,
                                lifetimes,
                                path,
                            }) = bound
                            {
                                let path = check_meta_path(&path);

                                // オリジナルのToTokensの実装を参考
                                let mut tokens = TokenStream::new();
                                let to_tokens = |tokens: &mut TokenStream| {
                                    modifier.to_tokens(tokens);
                                    lifetimes.to_tokens(tokens);
                                    path.to_tokens(tokens);
                                };
                                match &paren_token {
                                    Some(paren) => paren.surround(&mut tokens, to_tokens),
                                    None => to_tokens(&mut tokens),
                                }

                                TypeParamBound::Verbatim(tokens)
                            } else {
                                bound
                            }
                        })
                        .collect::<Punctuated<_, Token![+]>>();

                    WherePredicate::Type(PredicateType {
                        bounded_ty,
                        colon_token,
                        bounds: new_bounds,
                        lifetimes,
                    })
                }
                WherePredicate::Lifetime(_) => pred,
                _ => unreachable!(),
            }
        })
        .collect::<Punctuated<_, Token![,]>>();

    item_fn
}

// pub struct TestGenerics<const T: usize, S: Float> {
//     s: S,
// }

// const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
//     A
// }

/// from
///
/// const fn get_const_generics<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) {
/// }, $value, 0
///
/// to
///
/// const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
///     A
/// }
///
/// get_const_generics_a($value)
pub fn gen_get_const_generics(
    get_const_generics_fn_seed: ItemFn,
    value: Expr,
    num: usize,
) -> Option<Expr> {
    let get_const_generics_fn_seed = gen_get_const_generics_inner(get_const_generics_fn_seed, num)?;
    let fn_ident = get_const_generics_fn_seed.sig.ident.clone();

    let expr: Expr = parse_quote!({
        #get_const_generics_fn_seed

        #fn_ident(#value)
    });

    Some(expr)
}

pub fn gen_get_const_generics_inner(
    get_const_generics_fn_seed: ItemFn,
    num: usize,
) -> Option<ItemFn> {
    let mut get_const_generics_fn_seed = get_const_generics_fn_seed;

    let generics = get_const_generics_fn_seed.sig.generics.clone();
    let generics_arg = generics.params;
    let num_arg = generics_arg.get(num)?;

    let num_arg = match num_arg {
        GenericParam::Const(con) => con,
        _ => return None,
    };
    let ConstParam {
        ident: num_arg_ident,
        ty: num_arg_ty,
        ..
    } = num_arg;
    get_const_generics_fn_seed.sig.output =
        ReturnType::Type(Default::default(), Box::new(num_arg_ty.clone()));

    let stmts = &mut get_const_generics_fn_seed.block.stmts;
    *stmts = vec![Stmt::Expr(
        Expr::Verbatim(quote! {
            #num_arg_ident
        }),
        None,
    )];

    Some(get_const_generics_fn_seed)
}
