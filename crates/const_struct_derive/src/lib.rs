// https://astexplorer.net/

use std::f32::consts::E;

use convert_case::{Case, Casing as _};
use proc_macro::TokenStream as RawTokenStream;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, ItemConst};
use quote::quote;
use proc_macro2::TokenStream;

#[proc_macro_derive(ConstStruct)]
pub fn const_struct_derive(input: RawTokenStream) -> RawTokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = generate_const_struct_derive(input);
    match output {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_const_struct_derive(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Must be struct type",
            ))
        }
    };

    let mut field_names = fields.iter().map(|field| &field.ident);
    let mut field_types = fields.iter().map(|field| &field.ty);

    // Ok(quote! {
    //     impl #name {
    //         pub const fn new(#(#field_names: #field_types),*) -> Self {
    //             Self {
    //                 #(#field_names),*
    //             }
    //         }
    //     }
    // })

    let trait_name = quote::format_ident!("{}Ty", name);
    // to UPPER_SNAKE_CASE
    let field_names = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_snake = field_name.to_string();
        let field_name_upper_snake = field_name_snake.from_case(Case::Snake).to_case(Case::UpperSnake);
        quote::format_ident!("{}", field_name_upper_snake)
    });
    let const_field = field_names.zip(field_types).map(|(field, ty)| {
        quote! {
            const #field: #ty;
        }
    }).collect::<Vec<_>>();

    // let data_field =

    let new_trait = quote! {
        trait #trait_name {
            #(#const_field)*
        }
    };

    return Ok(quote! {
        #new_trait
    });
}

#[proc_macro_attribute]
pub fn const_struct(_attr: RawTokenStream, item: RawTokenStream) -> RawTokenStream {
    let input = parse_macro_input!(item as ItemConst);
    let output = generate_const_struct(input);
    match output {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_const_struct(input: ItemConst) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;
    let value = &input.expr;

    dbg!(&input);

    let ty_name = {
        let name_upper_snake = name.to_string();
        let name_pascal = name_upper_snake.from_case(Case::UpperSnake).to_case(Case::Pascal);
        quote::format_ident!("{}Ty", name_pascal)
    };

    let struct_define = quote! {
        struct #ty_name;
    };

    // let field_names = match &input.expr {
    //     syn::Expr::Struct(syn::ExprStruct { fields, .. }) => fields.iter().map(|field| {
    //         let field_name = field.member.as_ref().unwrap();
    //         let field_name_snake = field_name.to_string();
    //         let field_name_upper_snake = field_name_snake.from_case(Case::Snake).to_case(Case::UpperSnake);
    //         quote::format_ident!("{}", field_name_upper_snake)
    //     }).collect::<Vec<_>>(),
    //     _ => {
    //         return Err(syn::Error::new_spanned(
    //             input,
    //             "Must be struct type",
    //         ))
    //     }
    // };

    Ok(quote! {
        #input
        #struct_define
    })
}
