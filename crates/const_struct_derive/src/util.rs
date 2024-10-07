use parse::discouraged::Speculative as _;
use proc_macro2::{Spacing, TokenStream};
use punctuated::Punctuated;
use quote::{quote, ToTokens, TokenStreamExt as _};
use syn::*;
use FnArg::Typed;

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

    item_fn.sig.inputs.iter_mut().for_each(|input| {
        if let Typed(PatType { ty, .. }) = input {
            if let Type::Path(TypePath { path, .. }) = ty.as_mut() {
                let path = check_meta_path(&path);
                *ty = Box::new(Type::Verbatim(path));
            }
        }
    });

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

pub fn is_end_with_ty(path: &Path) -> bool {
    path.segments
        .last()
        .unwrap()
        .ident
        .to_string()
        .ends_with("Ty")
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
    ident_tys: Vec<TokenStream>,
    value: Expr,
    num: usize,
) -> Option<Expr> {
    let get_const_generics_fn_seed =
        gen_get_const_generics_inner(get_const_generics_fn_seed, ident_tys, num)?;
    let fn_ident = get_const_generics_fn_seed.sig.ident.clone();

    let expr: Expr = parse_quote!({
        #get_const_generics_fn_seed

        #fn_ident(#value)
    });

    Some(expr)
}

pub struct MetaPath {
    pub meta: Option<Token![$]>,
    pub path: Path,
}

impl parse::Parse for MetaPath {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let fork = input.fork();
        match fork.parse::<Token![$]>() {
            Ok(meta) => {
                let path: Path = fork.parse()?;
                input.advance_to(&fork);
                Ok(Self {
                    meta: Some(meta),
                    path,
                })
            }
            Err(_) => {
                let path: Path = input.parse()?;
                Ok(Self {
                    meta: None,
                    path,
                })
            }
        }
    }
}

impl ToTokens for MetaPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(meta) = &self.meta {
            meta.to_tokens(tokens);
        }
        self.path.to_tokens(tokens);
    }
}

pub fn gen_get_const_generics_inner(
    get_const_generics_fn_seed: ItemFn,
    ident_tys: Vec<TokenStream>,
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

    // let mut generics = &mut get_const_generics_fn_seed.sig.generics;
    let generics = get_const_generics_fn_seed.sig.generics;
    let params_clone = generics.params.clone();
    let rm_target_ident = generics
        .params
        .iter()
        // .cloned()
        // .zip(generics.where_clause.unwrap().predicates.iter().cloned())
        .filter_map(|param| match param {
            GenericParam::Type(TypeParam { ident, .. }) => Some(ident.clone()),
            GenericParam::Lifetime(_) => unimplemented!(),
            GenericParam::Const(_) => None,
        })
        .collect::<Vec<_>>();
    // println!("rm_target_ident: {:?}", rm_target_ident);
    let new_generics_param = generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Const(_)))
        .cloned()
        .collect::<Punctuated<_, Token![,]>>();
    let new_predicates = generics.where_clause.unwrap_or(WhereClause { where_token: Default::default(), predicates: Default::default() }).predicates.iter().filter(|predicate|
        if let WherePredicate::Type(PredicateType { bounded_ty, .. }) = predicate {
            let where_ident = if let Type::Path(TypePath { path, .. }) = bounded_ty {
                if let Some(PathSegment { ident, .. }) = path.segments.last() {
                    // println!("ident: {:?}", ident);
                    ident
                } else {
                    return true;
                }
            } else {
                return true;
            };
            !rm_target_ident.iter().any(|ident| ident == where_ident)
        } else {
            unimplemented!()
        }
    ).cloned().collect::<Punctuated<_, Token![,]>>();
    let generics = Generics {
        params: new_generics_param,
        where_clause: Some(WhereClause {
            where_token: Default::default(),
            predicates: new_predicates,
        }),
        ..generics
    };

    get_const_generics_fn_seed.sig.generics = generics;

    let mut meta_path = None;
    let mut change_meta_path_before = None;

    let input_args = if let Typed(PatType { ty, .. }) =
        get_const_generics_fn_seed.sig.inputs.get_mut(0).unwrap()
    {
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = ty.as_mut()
        {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                &mut segments.last_mut().unwrap().arguments
            {
                args
            } else {
                return None;
            }
        } else if let Type::Verbatim(token_stream) = ty.as_mut() {
            if let Ok(meta_path_) = parse2::<MetaPath>(token_stream.clone()) {
                meta_path = Some(meta_path_);
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                    &mut meta_path.as_mut().unwrap().path.segments.last_mut().unwrap().arguments
                {
                    change_meta_path_before = Some(|meta_path: MetaPath| {
                        *token_stream = quote! { #meta_path };
                    });
                    args
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else {
        return None;
    };

    input_args
        .iter_mut()
        .zip(params_clone.iter())
        .filter(|(_, param)| match param {
            GenericParam::Type(_) => true,
            GenericParam::Lifetime(_) => unimplemented!(),
            GenericParam::Const(_) => false,
        })
        .map(|(input_arg, _param)| input_arg)
        .zip(ident_tys.iter())
        .for_each(|(input_arg, ident_ty)| {
            if let GenericArgument::Type(type_) = input_arg {
                *type_ = Type::Verbatim(ident_ty.clone());
            }
        });

    if let Some(mut change_meta_path_before) = change_meta_path_before {
        change_meta_path_before(meta_path.unwrap());
    }

    Some(get_const_generics_fn_seed)
}
