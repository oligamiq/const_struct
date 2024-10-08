use std::{sync::Mutex, rc::Rc};

use parse::{discouraged::Speculative as _, Parse, ParseStream, Parser as _};
use proc_macro2::TokenStream;
use punctuated::Punctuated;
use quote::{format_ident, quote, ToTokens};
use syn::*;

use crate::{
    macro_alt::{default_primitive_macro_alt, struct_macro_alt},
    parse_value::{AdditionData, AdditionDataArgs},
    rewriter::change_macro::Switcher,
    util::{add_at_mark, gen_get_const_generics, is_end_with_ty},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Label {
    TupleStruct,
    VanillaStruct,
    Struct,
    Enum,
}

impl Parse for Label {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.parse::<Token![enum]>().is_ok() {
            return Ok(Self::Enum);
        }
        if input.parse::<Token![struct]>().is_ok() {
            return Ok(Self::Struct);
        }
        let ident = input.parse::<Ident>()?;

        match ident.to_string().as_str() {
            "tuple_struct" => Ok(Self::TupleStruct),
            "vanilla_struct" => Ok(Self::VanillaStruct),
            _ => Err(syn::Error::new(ident.span(), "expected a label")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericsData {
    pub _at: Token![@],
    pub ident: Ident,
    pub _paren_token: token::Paren,
    pub addition_data: AdditionDataArgs,
    pub _comma: Token![,],
    pub label: Label,
    pub _comma2: Token![,],
    pub const_fn: ItemFn,
}

impl ToTokens for GenericsData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            _at,
            ident,
            addition_data,
            _comma,
            label,
            _comma2,
            const_fn,
            ..
        } = self;
        let ident_with_at = add_at_mark(ident.clone());
        let label = match label {
            Label::TupleStruct => format_ident!("tuple_struct"),
            Label::VanillaStruct => format_ident!("vanilla_struct"),
            Label::Struct => format_ident!("struct"),
            Label::Enum => format_ident!("enum"),
        };
        tokens.extend(quote! {
            #ident_with_at(#addition_data, #label, #const_fn)
        });
    }
}

#[derive(Debug, Clone)]
pub enum TypeOrExpr {
    Type(Type),
    Expr(Expr),
}

impl Parse for TypeOrExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_try = input.fork();
        let expr = input.parse::<Expr>();
        if expr.is_ok() {
            return Ok(Self::Expr(expr?));
        }
        let ty = input_try.parse::<Type>();
        if ty.is_ok() {
            return Ok(Self::Type(input.parse()?));
        }
        Err(syn::Error::new(
            input.span(),
            "expected a type or an expression",
        ))
    }
}

impl ToTokens for TypeOrExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Type(ty) => ty.to_tokens(tokens),
            Self::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericInfo {
    pub correspondence: Vec<(Ident, TypeOrExpr)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstOrType {
    Const,
    Type,
}

impl GenericsData {
    pub fn get_generics_types(&self) -> Vec<GenericParam> {
        let const_fn = &self.const_fn;
        let gen = &const_fn.sig.generics;
        gen.params.iter().cloned().collect()
    }

    pub fn const_or_type(&self) -> Vec<ConstOrType> {
        let generics_types = self.get_generics_types();
        generics_types
            .iter()
            .map(|f| match f {
                GenericParam::Type(_) => ConstOrType::Type,
                GenericParam::Const(_) => ConstOrType::Const,
                _ => panic!("failed to get const_or_type"),
            })
            .collect()
    }

    pub fn get_ty_ident(&self) -> Ident {
        match self.label {
            Label::Struct => {
                let ident = self.ident.clone();
                if ident.to_string().ends_with("GetGenericsData") {
                    let ident = ident.to_string();
                    let ident = ident.split_at(ident.len() - "GetGenericsData".len()).0;
                    format_ident!("{}", ident)
                } else {
                    panic!("failed to get_ty_ident");
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl Parse for GenericsData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_try = input.fork();
        let _at = input_try.parse::<Token![@]>()?;
        let ident = input_try.parse::<Ident>()?;
        let content;
        let _paren_token = parenthesized!(content in input_try);
        let addition_data = content.parse::<AdditionDataArgs>()?;
        let _comma = content.parse::<Token![,]>()?;
        let label = content.parse::<Label>()?;
        let _comma2 = content.parse::<Token![,]>()?;
        let const_fn = content.parse::<ItemFn>()?;
        input.advance_to(&input_try);
        Ok(Self {
            _at,
            ident,
            _paren_token,
            addition_data,
            _comma,
            label,
            _comma2,
            const_fn,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ExpandCallFnWithGenericsArgs {
    pub addition_data: Option<AdditionDataArgs>,
    pub _comma: Option<Token![,]>,
    pub item: Punctuated<GenericsData, Token![,]>,
    pub call: MyExprCalls,
}

impl Parse for ExpandCallFnWithGenericsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // println!("ExpandCallFnWithGenericsArgs: input: {}", input);
        let addition_data = input.parse::<AdditionDataArgs>().ok();
        let _comma = if addition_data.is_some() {
            // println!("success to parse AdditionDataArgs");
            input.parse::<Token![,]>().ok()
        } else {
            // println!("failed to parse AdditionDataArgs");
            None
        };
        // println!("addition_data: {:#?}", addition_data);
        let mut item = Punctuated::new();
        loop {
            // println!("input: {}", input);
            if input.peek(Token![@]) {
                if let Ok(generics_data) = input.parse::<GenericsData>() {
                    // println!("success to parse GenericsData");
                    // println!("generics_data: {}", generics_data.to_token_stream());
                    // println!("item: {}", item.to_token_stream());
                    item.push_value(generics_data);
                    // println!("item: {}", item.to_token_stream());

                    // println!("input1: {}", input);
                    if let Ok(_comma) = input.parse::<Token![,]>() {
                        item.push_punct(_comma);

                        // println!("input2: {}", input);
                    } else {
                        eprintln!("failed to parse Token![,]");
                        unreachable!();
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // println!("item: {}", item.to_token_stream());

        let call = input.parse::<MyExprCalls>()?;
        // println!("success to parse MyExprCalls");
        Ok(Self {
            addition_data,
            _comma,
            item,
            call,
        })
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
    let hash = crate::str_hash(&input.clone().to_string());

    // println!("input_with_data: {}", input.to_token_stream());

    let input_with_data = parse2::<ExpandCallFnWithGenericsArgs>(input)?;

    // println!("input_with_data success");

    // dbg!(&input_with_data);

    // println!("input_with_data2: {:#?}", input_with_data);

    let ExpandCallFnWithGenericsArgs {
        addition_data: default_addition_data_args,
        _comma,
        item: addition_define_data,
        call: mut input,
    } = input_with_data;
    let default_addition_data_args_clone = default_addition_data_args.clone().unwrap_or_default();
    let default_addition_data: AdditionData = default_addition_data_args
        .map(|args| args.into())
        .unwrap_or_default();

    // println!("define_data: {:#?}", define_data);

    let input_clone = input.clone();
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
        let extend = Rc::new(Mutex::new(Vec::<GenericArgument>::new()));
        // let mut switched_arg = arg.clone();
        let switched_arg = Rc::new(Mutex::new(arg.clone()));
        let return_data = Rc::new(Mutex::new(None));

        // println!("arg: {}", arg.to_token_stream());

        arg.clone().switcher(&|mac| {
            if return_data.lock().unwrap().is_some() {
                return mac.to_token_stream();
            }

            // check about the macro: F32, F64, etc.
            // println!("mac: {}", mac.to_token_stream());

            if let Some(ty) = crate::ident::gen_primitive_ty(&mac.path.segments.last().unwrap().ident) {
                let ty = ty(parse2::<Expr>(mac.tokens.clone()).unwrap());
                // println!("ty: {}", ty.to_token_stream());

                let switcher = |inner_mac: Macro| -> TokenStream {
                    if inner_mac == mac {
                        ty.to_token_stream()
                    } else {
                        inner_mac.to_token_stream()
                    }
                };

                let switched_arg_clone = switched_arg.clone().lock().unwrap().clone();
                *switched_arg.lock().unwrap() = switched_arg_clone.switcher(&switcher);

                return mac.to_token_stream();
            }

            let mac = mac.clone();
            let tokens = mac.tokens.clone();
            // println!("failed?");
            let args = Punctuated::<Expr, Token![,]>::parse_terminated
                .parse2(tokens)
                .unwrap_or_else(|e| {
                    eprintln!("failed!!! {}", e);
                    panic!();
                });
            // println!("not failed: {}", args.to_token_stream());

            // dbg!(&args);

            let macro_name = mac.path.segments.last().unwrap().ident.to_string();

            let exist_define_data = addition_define_data.iter().any(|data| {
                data.ident == format!("{macro_name}GetGenericsData")
            });
            if !exist_define_data {
                // println!("q0:");
                let get_generics_data = add_at_mark(format_ident!("{macro_name}GetGenericsData"));
                // println!("q1: {:#?}", get_generics_data);
                let self_macro = mac.path.clone();
                let call_with_generics_path = default_addition_data.get_changed_path_from_quote(quote! {
                    ::const_struct::call_with_generics
                });
                // println!("addition_data: {:#?}", addition_data);
                let q = quote! { #self_macro!(#get_generics_data, #default_addition_data_args_clone, #call_with_generics_path, #addition_define_data #input_clone) };
                // println!("q: {}", q.to_token_stream());
                *return_data.lock().unwrap() = Some(q);
                return mac.to_token_stream();
            }
            let define_data = addition_define_data.iter().find(|data| {
                data.ident == format!("{macro_name}GetGenericsData")
            }).unwrap();
            let addition_data = default_addition_data.clone().extend(define_data.addition_data.clone().into());

            // println!("try get args_last");
            let args_last = args.last().unwrap().clone();
            // println!("get args_last: {}", args_last.to_token_stream());

            // outer declarationの場合
            let (is_outer_declaration, ty_path) = {
                let last = args.last().unwrap();
                let last_token = last.to_token_stream().to_string();
                if let Ok(path) = parse_str::<Path>(&last_token) {
                    if is_end_with_ty(&path) {
                        (true, Some(path))
                    } else {
                        (false, None)
                    }
                } else {
                    (false, None)
                }
            };

            let const_or_type = define_data.const_or_type();

            let type_num = const_or_type
                .iter()
                .filter(|const_or_type| matches!(const_or_type, ConstOrType::Type))
                .count();
            let args_len = args.len();

            let mut args_iter = args.clone().into_iter();
            let ident_tys = if args_len == const_or_type.len() + 1 {
                const_or_type
                .iter()
                .filter_map(|const_or_type| {
                    if let ConstOrType::Type = const_or_type {
                        let arg = args_iter.next().unwrap();
                        Some(arg.to_token_stream())
                    } else {
                        args_iter.next();
                        None
                    }
                })
                .collect::<Vec<TokenStream>>()
            } else if args.len() == 1 {
                Vec::new()
            } else if args.len() == type_num + 1 {
                const_or_type
                .iter()
                .filter_map(|const_or_type| {
                    if let ConstOrType::Type = const_or_type {
                        let arg = args_iter.next().unwrap();
                        Some(arg.to_token_stream())
                    } else {
                        None
                    }
                })
                .collect::<Vec<TokenStream>>()
            } else {
                panic!("failed to parse Argument");
            };
            let get_generics = |num: usize, value: Expr| {
                //     let mut mac = mac.clone();
                //     let macro_first_arg =
                //         add_at_mark(format_ident!("{macro_name}GetInnerGenerics{num}"));
                //     mac.tokens = quote! { #macro_first_arg, #value };
                //     mac
                gen_get_const_generics(define_data.const_fn.clone(), ident_tys.clone(), value, num)
            };

            let infer_process = |num| {
                if is_outer_declaration {
                    let const_or_type = match const_or_type.get(num) {
                        Some(const_or_type) => const_or_type,
                        None => panic!("failed to get const_or_type"),
                    };
                    let ty_path = ty_path.clone().unwrap();
                    match const_or_type {
                        ConstOrType::Const => {
                            let keep_type_const_path =
                                addition_data.get_changed_path_from_quote(quote! {
                                    ::const_struct::keeptype::KeepTypeConst
                                });
                            let ty: GenericArgument = parse_quote!({
                                <#ty_path as #keep_type_const_path<#num>>::N
                            });
                            ty
                        }
                        ConstOrType::Type => {
                            let keep_type_path =
                                addition_data.get_changed_path_from_quote(quote! {
                                    ::const_struct::keeptype::KeepType
                                });
                            let ty: GenericArgument =
                                parse_quote!(<#ty_path as #keep_type_path<#num>>::Type);
                            ty
                        }
                    }
                } else {
                    let expr = get_generics(num, args_last.clone());
                    GenericArgument::Const(expr.expect("failed to get_generics"))
                }
            };

            let new_generic = if args_len == const_or_type.len() + 1 {
                args.into_iter()
                    .enumerate()
                    .filter(|(i, _)| *i < args_len - 1)
                    .map(|(num, arg)| match arg {
                        Expr::Infer(_) => infer_process(num),
                        _ => {
                            let str = arg.to_token_stream().to_string();
                            // println!("str: {}", str);
                            match parse_str::<GenericArgument>(&str) {
                                Ok(generics) => generics,
                                Err(_) => panic!("failed to parse Argument"),
                            }
                        }
                    })
                    .collect::<Vec<GenericArgument>>()
            } else if args_len == 1 {
                const_or_type
                    .iter()
                    .enumerate()
                    .map(|(num, _)| infer_process(num))
                    .collect::<Vec<GenericArgument>>()
            } else if args_len == type_num + 1 {
                let mut args = args.into_iter();
                const_or_type
                    .iter()
                    .enumerate()
                    .map(|(num, const_or_type)| {
                        if let ConstOrType::Type = const_or_type {
                            let arg = args.next().unwrap();
                            let str = arg.to_token_stream().to_string();
                            match parse_str::<GenericArgument>(&str) {
                                Ok(generics) => generics,
                                Err(_) => panic!("failed to parse Argument"),
                            }
                        } else {
                            infer_process(num)
                        }
                    })
                    .collect::<Vec<GenericArgument>>()
            } else {
                panic!("failed to parse Argument");
            };

            let switcher = |inner_mac: Macro| -> TokenStream {
                if inner_mac == mac {
                    if is_outer_declaration {
                        return GenericArgument::Type(Type::Path(TypePath {
                            qself: None,
                            path: ty_path.clone().unwrap(),
                        }))
                        .to_token_stream();
                    }
                    let ty = struct_macro_alt(
                        addition_data.clone(),
                        ident_tys.clone(),
                        define_data.clone(),
                        new_generic.clone(),
                        hash,
                    );
                    let ty = ty(inner_mac.tokens.clone()).unwrap();
                    ty.to_token_stream()
                } else {
                    inner_mac.to_token_stream()
                }
            };

            let switched_arg_clone = switched_arg.clone().lock().unwrap().clone();
            *switched_arg.lock().unwrap() = switched_arg_clone.switcher(&switcher);

            extend.lock().unwrap().extend(new_generic);

            mac.to_token_stream()
        });

        let extend = extend.lock().unwrap().clone();

        // println!("extend: {}", quote! { #( #extend )* });
        // println!("switched_arg: {}", switched_arg.lock().unwrap().clone().to_token_stream());

        if let Some(return_data) = return_data.lock().unwrap().clone() {
            return Ok(return_data);
        }

        new_generics.extend(extend);
        let switched_arg = switched_arg.lock().unwrap().clone();
        new_generics.push(switched_arg);
    }

    // println!("new_generics: {}", new_generics.to_token_stream());

    let new_generics = new_generics.switcher(&|mac| {
        if let Some(path) = default_addition_data.get_addition_data(&mac.path) {
            let mut mac = mac;
            mac.path = path;
            mac.to_token_stream()
        } else {
            default_primitive_macro_alt(mac)
        }
    });

    *generics = new_generics;

    // println!("call_with_generics output: {}", input.to_token_stream());

    // let switcher

    Ok(input.into_token_stream())
}
