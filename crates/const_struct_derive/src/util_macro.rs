use parse::{discouraged::Speculative as _, Parse, ParseStream, Parser as _};
use proc_macro2::TokenStream;
use punctuated::Punctuated;
use quote::{format_ident, quote, ToTokens};
use syn::*;

#[derive(Debug, Clone)]
pub enum ConstOrType {
    Const,
    Type,
}

impl Parse for ConstOrType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // println!("ConstOrType input: {}", input);
        match input.parse::<Token![const]>() {
            Ok(_) => Ok(Self::Const),
            Err(_) => {
                let _type = input.parse::<Token![type]>()?;
                Ok(Self::Type)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericsData {
    pub ident: Ident,
    pub _paren_token: token::Paren,
    pub const_or_type: Punctuated<ConstOrType, Token![,]>,
}

impl Parse for GenericsData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_try = input.fork();
        // println!("input_try: {}", input_try);
        let ident = input_try.parse::<Ident>()?;
        // println!("ident: {}", ident);
        let content;
        let _paren_token = parenthesized!(content in input_try);
        // println!("content: {}", content);
        let const_or_type = Punctuated::<ConstOrType, Token![,]>::parse_terminated(&content)?;
        // println!("const_or_type: {:?}", const_or_type);
        input.advance_to(&input_try);
        Ok(Self {
            ident,
            _paren_token,
            const_or_type,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ExpandCallFnWithGenericsArgs {
    pub item: Option<GenericsData>,
    pub _comma: Option<Token![,]>,
    pub call: MyExprCalls,
}

impl Parse for ExpandCallFnWithGenericsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<GenericsData>() {
            Ok(item) => {
                // println!("success to parse GenericsData");
                let _comma = input.parse::<Token![,]>().ok();
                // println!("success to parse Token![,]");
                let call = input.parse::<MyExprCalls>()?;
                // println!("success to parse MyExprCalls");
                Ok(Self {
                    item: Some(item),
                    _comma,
                    call,
                })
            }
            Err(_) => {
                // println!("failed to parse GenericsData");
                let call = input.parse::<MyExprCalls>()?;
                // println!("success to parse MyExprCalls");
                Ok(Self {
                    item: None,
                    _comma: None,
                    call,
                })
            }
        }
    }
}

// example
// call_tester::<TestGenerics!(56, f32, TestGenerics { s: 0.6, t: [0; 56] })>()
#[derive(Debug, Clone)]
pub enum MyExprCalls {
    Call(ExprCall),
    MethodCall(ExprMethodCall),
}

impl Parse for MyExprCalls {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // println!("MyExprCalls input: {}", input);
        match input.parse::<ExprCall>() {
            Ok(call) => {
                // println!("MyExprCalls success to parse ExprCall");
                Ok(Self::Call(call))
            }
            Err(_) => {
                // println!("MyExprCalls failed to parse ExprCall");
                // println!("MyExprCalls error: {}", e);
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
    // println!("input_with_data: {}", input.to_token_stream());

    let input_with_data = parse2::<ExpandCallFnWithGenericsArgs>(input)?;

    // println!("input_with_data2: {:#?}", input_with_data);

    let ExpandCallFnWithGenericsArgs {
        item: define_data,
        call: mut input,
        ..
    } = input_with_data;

    println!("define_data: {:#?}", define_data);

    let generics = input.generics_mut_ref().ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected a function call with generics",
        )
    })?;

    // dbg!(&generics);
    // println!("generics: {}", generics.to_token_stream());

    let mut new_generics: Punctuated<GenericArgument, Token![,]> = Punctuated::new();

    for arg in &*generics {
        let extend = match &arg {
            GenericArgument::Type(Type::Macro(mac)) => {
                let mac = mac.mac.clone();
                let tokens = mac.tokens.clone();
                let args = Punctuated::<Expr, Token![,]>::parse_terminated
                    .parse2(tokens)
                    .unwrap();

                let macro_name = mac.path.segments.last().unwrap().ident.to_string();

                let mut exist_define_data = false;
                if let Some(define_data) = define_data.clone() {
                    if define_data.ident.to_string() == format!("{macro_name}GetGenericsData") {
                        exist_define_data = true;
                    }
                }
                if !exist_define_data {
                    let get_generics_data = format_ident!("{macro_name}GetGenericsData");
                    let self_macro = mac.path.clone();
                    let q = quote! { #self_macro!(#get_generics_data, ::const_struct_derive::call_with_generics, #input) };
                    println!("q: {}", q);
                    return Ok(
                        quote! { #self_macro!(#get_generics_data, ::const_struct_derive::call_with_generics, #input) }.into(),
                    );
                }

                let get_generics = |num: usize, middle: &str, value: Expr| {
                    let mut mac = mac.clone();
                    let macro_first_arg = format!("{macro_name}{middle}{num}");
                    let macro_first_arg =
                        Ident::new(&macro_first_arg, proc_macro2::Span::call_site());
                    mac.tokens = quote! { #macro_first_arg, #value };
                    mac
                };

                let args_last = args.last().unwrap().clone();

                // outer declarationの場合
                let (is_outer_declaration, ty_path) = {
                    let last = args.last().unwrap();
                    let last_token = last.to_token_stream().to_string();
                    if let Ok(path) = parse_str::<Path>(&last_token) {
                        if path
                            .segments
                            .last()
                            .unwrap()
                            .ident
                            .to_string()
                            .ends_with("Ty")
                        {
                            (true, Some(path))
                        } else {
                            (false, None)
                        }
                    } else {
                        (false, None)
                    }
                };
                let infer_process = |num| {
                    if is_outer_declaration {
                        let const_or_type =
                            match define_data.as_ref().unwrap().const_or_type.get(num) {
                                Some(const_or_type) => const_or_type,
                                None => panic!("failed to get const_or_type"),
                            };
                        let ty_path = ty_path.clone().unwrap();
                        match const_or_type {
                            ConstOrType::Const => {
                                let ty: GenericArgument = parse_quote!({
                                    <KeepTypeStruct<#ty_path, #num> as KeepType>::Type::__DATA
                                });
                                ty
                            }
                            ConstOrType::Type => {
                                let ty: GenericArgument = parse_quote!(<KeepTypeStruct<#ty_path, #num> as KeepType>::Type);
                                ty
                            }
                        }
                    } else {
                        let mac = get_generics(num, "GetInnerGenerics", args_last.clone());
                        let mac = GenericArgument::Const(Expr::Macro(ExprMacro {
                            mac,
                            attrs: Vec::new(),
                        }));
                        mac
                    }
                };

                let args_len = args.len();
                let mut new_generic = args
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| *i < args_len - 1)
                    .map(|(num, arg)| match arg {
                        Expr::Infer(_) => infer_process(num),
                        _ => {
                            let str = arg.to_token_stream().to_string();
                            // println!("str: {}", str);
                            let generics = match parse_str::<GenericArgument>(&str) {
                                Ok(generics) => generics,
                                Err(_) => panic!("failed to parse Argument"),
                            };
                            generics
                        }
                    })
                    .collect::<Vec<GenericArgument>>();

                new_generic.push(if is_outer_declaration {
                    GenericArgument::Type(Type::Path(TypePath {
                        qself: None,
                        path: ty_path.unwrap(),
                    }))
                } else {
                    arg.clone()
                });

                // println!("new_generic: {}", quote! { #(#new_generic),* });

                new_generic
            }
            _ => vec![arg.clone()],
        };

        new_generics.extend(extend);
    }

    // println!("new_generics: {}", new_generics.to_token_stream());

    *generics = new_generics;

    Ok(input.into_token_stream())
}
