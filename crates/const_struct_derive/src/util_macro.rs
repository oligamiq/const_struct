use parse::{discouraged::Speculative as _, Parse, ParseStream, Parser as _};
use proc_macro2::TokenStream;
use punctuated::Punctuated;
use quote::{format_ident, quote, ToTokens};
use syn::*;

use crate::{parse_value::TyAndExpr, rewriter::change_macro::Switcher, util::add_at_mark};

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
    pub _comma2: Token![,],
    pub macros: Punctuated<Macro, Token![,]>,
}

pub enum TypeOrExpr {
    Type(Type),
    Expr(Expr),
}

impl ToTokens for TypeOrExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Type(ty) => ty.to_tokens(tokens),
            Self::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

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

    pub fn get_struct_ident(&self) -> Ident {
        self.ident.clone()
    }

    pub fn get_parsed_value(
        &self,
        num: usize,
        expr_arg: Expr,
        generic_info: &GenericInfo,
    ) -> Result<Type> {
        let macro_num = self.macros.iter().nth(num).unwrap();
        if macro_num.path.is_ident("parse_value") {
            let tokens = macro_num.tokens.clone();
            let TyAndExpr {
                ty,
                expr,
                additional_data,
                ..
            } = parse2::<TyAndExpr>(tokens)?;
            let additional_data = additional_data.unwrap_or_default();
            let change_expr = |mac: Macro| {
                if mac.path.is_ident("expr") {
                    expr_arg.to_token_stream()
                } else {
                    mac.to_token_stream()
                }
            };
            let expr = expr.switcher(&change_expr);
            let change_ty = |mac: Macro| {
                if mac.path.is_ident("gen") {
                    let ident = parse::<Ident>(mac.tokens.clone().into()).unwrap();
                    let ty_or_expr = &generic_info
                        .correspondence
                        .iter()
                        .find(|(ident2, _)| ident == *ident2)
                        .unwrap()
                        .1;
                    ty_or_expr.to_token_stream()
                } else {
                    mac.to_token_stream()
                }
            };
            let ty = ty.switcher(&change_ty);
            let ret_ty = crate::parse_value::parse_value(ty, expr, &additional_data.into())?;

            Ok(ret_ty)
        } else {
            panic!("failed to get_parsed_value");
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
        let _comma2 = content.parse::<Token![,]>()?;
        let macros = Punctuated::<Macro, Token![,]>::parse_terminated(&content)?;
        input.advance_to(&input_try);
        Ok(Self {
            _at,
            ident,
            _paren_token,
            label,
            _comma,
            const_fn,
            _comma2,
            macros,
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
                println!("success to parse GenericsData");
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
            Err(e) => {
                println!("failed to parse GenericsData: {}", e);
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

    // println!("input_with_data success");

    // dbg!(&input_with_data);

    // println!("input_with_data2: {:#?}", input_with_data);

    let ExpandCallFnWithGenericsArgs {
        item: define_data,
        call: mut input,
        ..
    } = input_with_data;

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
                        println!("failed!!! {}", e);
                        panic!();
                    });
                // println!("not failed: {}", args.to_token_stream());

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
                    // let q = quote! { #self_macro!(#get_generics_data, ::const_struct_derive::call_with_generics, #input) };
                    // println!("q: {}", q);
                    return Ok(
                        quote! { #self_macro!(#get_generics_data, ::const_struct_derive::call_with_generics, #input) }.into(),
                    );
                }
                let define_data = define_data.as_ref().unwrap();

                let get_generics = |num: usize, value: Expr| {
                    let mut mac = mac.clone();
                    let macro_first_arg =
                        add_at_mark(format_ident!("{macro_name}GetInnerGenerics{num}"));
                    mac.tokens = quote! { #macro_first_arg, #value };
                    mac
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
                                let ty: GenericArgument = parse_quote!({
                                    <#ty_path as KeepTypeConst<#num>>::N
                                });
                                ty
                            }
                            ConstOrType::Type => {
                                let ty: GenericArgument =
                                    parse_quote!(<#ty_path as KeepType<#num>>::Type);
                                ty
                            }
                        }
                    } else {
                        let mac = get_generics(num, args_last.clone());
                        let mac = GenericArgument::Const(Expr::Macro(ExprMacro {
                            mac,
                            attrs: Vec::new(),
                        }));
                        mac
                    }
                };

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
                                let generics = match parse_str::<GenericArgument>(&str) {
                                    Ok(generics) => generics,
                                    Err(_) => panic!("failed to parse Argument"),
                                };
                                // println!("success: str");
                                generics
                            }
                        })
                        .collect::<Vec<GenericArgument>>()
                } else if args.len() == 1 {
                    const_or_type
                        .iter()
                        .enumerate()
                        .map(|(num, _)| infer_process(num))
                        .collect::<Vec<GenericArgument>>()
                } else {
                    panic!("failed to parse Argument");
                };

                new_generic.push(if is_outer_declaration {
                    GenericArgument::Type(Type::Path(TypePath {
                        qself: None,
                        path: ty_path.unwrap(),
                    }))
                } else {
                    arg.clone()
                });

                println!("new_generic: {}", quote! { #(#new_generic),* });

                new_generic
            }
            _ => vec![arg.clone()],
        };

        new_generics.extend(extend);
    }

    // println!("new_generics: {}", new_generics.to_token_stream());

    *generics = new_generics;

    println!("input: {}", input.to_token_stream());

    Ok(input.into_token_stream())
}
