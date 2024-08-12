use parse::{Parse, ParseStream, Parser as _};
use proc_macro::TokenStream as RawTokenStream;
use proc_macro2::{TokenStream, TokenTree};
use punctuated::Punctuated;
use quote::{quote, ToTokens};
use syn::*;
use token::Paren;

pub struct TtAndTt {
    input: Expr,
    _comma: Token![,],
    tt_is_underscore: Expr,
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

// example
// call_tester::<TestGenerics!(56, f32, TestGenerics { s: 0.6, t: [0; 56] })>()
#[derive(Debug)]
enum MyExprCalls {
    Call(ExprCall),
    MethodCall(ExprMethodCall),
}

impl Parse for MyExprCalls {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<ExprCall>() {
            Ok(call) => Ok(Self::Call(call)),
            Err(_) => {
                let method_call = input.parse::<ExprMethodCall>()?;
                Ok(Self::MethodCall(method_call))
            }
        }
    }
}

impl ToTokens for MyExprCalls {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Call(call) => call.to_tokens(tokens),
            Self::MethodCall(method_call) => method_call.to_tokens(tokens),
        }
    }
}

impl MyExprCalls {
    fn generics_mut_ref(&mut self) -> Option<&mut Punctuated<GenericArgument, Token![,]>> {
        match self {
            Self::Call(call) => {
                let func = &mut call.func;
                let path = match func.as_mut() {
                    Expr::Path(path) => path,
                    _ => return None,
                };
                let segments = &mut path.path.segments;
                let last_segment = segments.last_mut()?;
                let arguments = &mut last_segment.arguments;
                let generic_argument = match arguments {
                    PathArguments::AngleBracketed(args) => &mut args.args,
                    _ => return None,
                };
                Some(generic_argument)
            }
            Self::MethodCall(method_call) => {
                let turbofish = &mut method_call.turbofish;
                match turbofish {
                    Some(AngleBracketedGenericArguments { args, .. }) => Some(args),
                    None => None,
                }
            }
        }
    }
}

pub fn expand_call_fn_with_generics(input: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<MyExprCalls>(input)?;

    let generics = input.generics_mut_ref().ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected a function call with generics",
        )
    })?;

    // dbg!(&generics);
    // println!("generics: {}", generics.to_token_stream());

    let new_generics = generics
        .iter()
        .flat_map(|arg| match &arg {
            GenericArgument::Type(Type::Macro(mac)) => {
                let mac = mac.mac.clone();
                let tokens = mac.tokens.clone();
                let get_const_generics = |num: String, value: Expr| {
                    let mut mac = mac.clone();
                    let macro_name = mac.path.segments.last().unwrap().ident.to_string();
                    let macro_name = format!("{}GetConstGenerics{num}", macro_name);
                    let macro_name = Ident::new(&macro_name, proc_macro2::Span::call_site());
                    mac.tokens = quote! { #macro_name, #value };
                    mac
                };
                let args = Punctuated::<Expr, Token![,]>::parse_terminated
                    .parse2(tokens)
                    .unwrap();

                let args_last = args.last().unwrap().clone();

                let args_len = args.len();
                let mut new_generic = args
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| *i < args_len - 1)
                    .filter_map(|(num, arg)| match arg {
                        Expr::Infer(_) => {
                            let num = num.to_string();
                            let mac = get_const_generics(num, args_last.clone());
                            let mac = GenericArgument::Const(Expr::Macro(ExprMacro {
                                mac,
                                attrs: Vec::new(),
                            }));
                            println!("mac: {}", mac.to_token_stream());
                            Some(mac)
                        }
                        _ => {
                            let str = arg.to_token_stream().to_string();
                            println!("str: {}", str);
                            let generics = match parse_str::<GenericArgument>(&str) {
                                Ok(generics) => generics,
                                Err(_) => return None,
                            };
                            Some(generics)
                        }
                    })
                    .collect::<Vec<GenericArgument>>();

                new_generic.push(arg.clone());

                return new_generic;
            }
            _ => return vec![arg.clone()],
        })
        .collect::<Punctuated<GenericArgument, Token![,]>>();

    // println!("new_generics: {}", new_generics.to_token_stream());

    *generics = new_generics;

    Ok(input.into_token_stream())
}
