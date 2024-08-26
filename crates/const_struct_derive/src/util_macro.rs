use parse::{discouraged::Speculative as _, Parse, ParseStream, Parser as _};
use proc_macro2::TokenStream;
use punctuated::Punctuated;
use quote::{format_ident, quote, ToTokens};
use syn::*;

use crate::{
    macro_alt::{default_primitive_macro_alt, struct_macro_alt},
    parse_value::{AdditionData, AdditionDataArgs},
    rewriter::change_macro::Switcher,
    util::{add_at_mark, gen_get_const_generics},
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
        if let Ok(_) = input.parse::<Token![enum]>() {
            return Ok(Self::Enum);
        }
        if let Ok(_) = input.parse::<Token![struct]>() {
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
    pub label: Label,
    pub _comma: Token![,],
    pub const_fn: ItemFn,
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

#[derive(Debug, Clone)]
pub enum ConstOrType {
    Const,
    Type,
}

impl GenericsData {
    pub fn get_generics_types(&self) -> Vec<GenericParam> {
        let const_fn = &self.const_fn;
        let gen = &const_fn.sig.generics;
        gen.params.iter().map(|f| f.clone()).collect()
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
        let label = content.parse::<Label>()?;
        let _comma = content.parse::<Token![,]>()?;
        let const_fn = content.parse::<ItemFn>()?;
        input.advance_to(&input_try);
        Ok(Self {
            _at,
            ident,
            _paren_token,
            label,
            _comma,
            const_fn,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ExpandCallFnWithGenericsArgs {
    pub addition_data: Option<AdditionData>,
    pub item: Option<GenericsData>,
    pub _comma: Option<Token![,]>,
    pub call: MyExprCalls,
}

impl Parse for ExpandCallFnWithGenericsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // println!("ExpandCallFnWithGenericsArgs: input: {}", input);
        let addition_data = input.parse::<AdditionDataArgs>().ok();
        let addition_data = addition_data.map(|data| data.into());
        if let Some(_) = addition_data {
            // println!("success to parse AdditionDataArgs");
            let _comma = input.parse::<Token![,]>()?;
        }
        match input.parse::<GenericsData>() {
            Ok(item) => {
                // println!("success to parse GenericsData");
                let _comma = input.parse::<Token![,]>().ok();
                // println!("success to parse Token![,]");
                let call = input.parse::<MyExprCalls>()?;
                // println!("success to parse MyExprCalls");
                Ok(Self {
                    addition_data,
                    item: Some(item),
                    _comma,
                    call,
                })
            }
            Err(_e) => {
                // eprintln!("failed to parse GenericsData: {}", _e);
                // println!("failed to parse GenericsData");
                let call = input.parse::<MyExprCalls>()?;
                // println!("success to parse MyExprCalls");
                Ok(Self {
                    addition_data,
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

    // println!("input_with_data success");

    // dbg!(&input_with_data);

    // println!("input_with_data2: {:#?}", input_with_data);

    let ExpandCallFnWithGenericsArgs {
        item: define_data,
        call: mut input,
        addition_data,
        ..
    } = input_with_data;
    let addition_data = addition_data.unwrap_or_default();

    // println!("define_data: {:#?}", define_data);

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

                let mut exist_define_data = false;
                if let Some(define_data) = define_data.clone() {
                    if define_data.ident.to_string() == format!("{macro_name}GetGenericsData") {
                        exist_define_data = true;
                    }
                }
                if !exist_define_data {
                    // println!("q0:");
                    let get_generics_data =
                        add_at_mark(format_ident!("{macro_name}GetGenericsData"));
                    // println!("q1: {:#?}", get_generics_data);
                    let self_macro = mac.path.clone();
                    let call_with_generics_path =
                        addition_data.get_changed_path_from_quote(quote! {
                            ::const_struct::call_with_generics
                        });
                    // println!("addition_data: {:#?}", addition_data);
                    // println!("q: {}", quote! { #self_macro!(#get_generics_data, #call_with_generics_path, #input) });
                    return Ok(
                        quote! { #self_macro!(#get_generics_data, #call_with_generics_path, #input) }.into(),
                    );
                }
                let define_data = define_data.as_ref().unwrap();

                let get_generics = |num: usize, value: Expr| {
                    //     let mut mac = mac.clone();
                    //     let macro_first_arg =
                    //         add_at_mark(format_ident!("{macro_name}GetInnerGenerics{num}"));
                    //     mac.tokens = quote! { #macro_first_arg, #value };
                    //     mac
                    gen_get_const_generics(define_data.const_fn.clone(), value, num)
                };

                // println!("try get args_last");
                let args_last = args.last().unwrap().clone();
                // println!("get args_last: {}", args_last.to_token_stream());

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

                let const_or_type = define_data.const_or_type();

                let infer_process = |num| {
                    if is_outer_declaration {
                        let const_or_type = match const_or_type.get(num) {
                            Some(const_or_type) => const_or_type,
                            None => panic!("failed to get const_or_type"),
                        };
                        let ty_path = ty_path.clone().unwrap();
                        match const_or_type {
                            ConstOrType::Const => {
                                let keep_type_const_path = addition_data
                                    .get_changed_path_from_quote(quote! {
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
                        let mac = GenericArgument::Const(expr.expect("failed to get_generics"));
                        mac
                    }
                };

                let type_num = const_or_type
                    .iter()
                    .filter(|const_or_type| matchs!(const_or_type, ConstOrType::Type))
                    .count();
                let args_len = args.len();
                let mut new_generic = if args_len == const_or_type.len() + 1 {
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

                let new_generic_only_type = new_generic.clone();

                new_generic.push(if is_outer_declaration {
                    GenericArgument::Type(Type::Path(TypePath {
                        qself: None,
                        path: ty_path.unwrap(),
                    }))
                } else {
                    // println!("arg: {}", args_last.to_token_stream());

                    arg.clone()
                });

                // println!("new_generic: {}", quote! { #(#new_generic),* });

                let switcher = |inner_mac: Macro| -> TokenStream {
                    if inner_mac.path == mac.path {
                        let ty = struct_macro_alt(
                            addition_data.clone(),
                            define_data.clone(),
                            new_generic_only_type.clone(),
                        );
                        let ty = ty(inner_mac.tokens.clone()).unwrap();
                        ty.to_token_stream()
                    } else {
                        inner_mac.to_token_stream()
                    }
                };

                let new_generic = new_generic.switcher(&switcher);

                // println!("new_generic_switcher: {}", quote! { #(#new_generic),* });

                new_generic
            }
            _ => vec![arg.clone()],
        };

        new_generics.extend(extend);
    }

    // println!("new_generics: {}", new_generics.to_token_stream());

    let new_generics = new_generics.switcher(&|mac| {
        if let Some(path) = addition_data.get_addition_data(&mac.path) {
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
