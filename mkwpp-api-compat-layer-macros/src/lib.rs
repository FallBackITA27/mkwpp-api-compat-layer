use std::hint::unreachable_unchecked;

use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Ident, parse::Parse, parse_macro_input};

use crate::{derive_endpoint::EndpointArgs, derive_input_from_actix::QueryArgs};

mod derive_endpoint;
mod derive_getters;
mod derive_input_from_actix;
mod to_scope;
mod utils;

#[proc_macro_derive(GetId, attributes(internal))]
pub fn derive_get_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_getters::derive_getter(
        input,
        "id",
        Ident::new("GetId", Span::call_site()),
        Ident::new("HasId", Span::call_site()),
        Ident::new("get_id", Span::call_site()),
        syn::Type::Verbatim(quote! { i32 }),
        false,
    )
}

#[proc_macro_derive(GetCategory, attributes(internal))]
pub fn derive_get_category(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_getters::derive_getter(
        input,
        "category",
        Ident::new("GetCategory", Span::call_site()),
        Ident::new("HasCategory", Span::call_site()),
        Ident::new("get_category", Span::call_site()),
        syn::Type::Verbatim(quote! { crate::common_types::category::Category }),
        false,
    )
}

#[proc_macro_derive(GetSessionToken, attributes(internal))]
pub fn derive_get_session_token(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_getters::derive_getter(
        input,
        "session_token",
        Ident::new("GetSessionToken", Span::call_site()),
        Ident::new("HasSessionToken", Span::call_site()),
        Ident::new("get_session_token", Span::call_site()),
        syn::Type::Verbatim(quote! { &str }),
        true,
    )
}

#[proc_macro_derive(Endpoint, attributes(internal))]
pub fn derive_endpoint(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as syn::ItemStruct);

    let mut args = EndpointArgs::default();

    for attr in &input_struct.attrs {
        if utils::attribute_is_internal(attr) {
            args = attr.parse_args().expect("Couldn't parse args");
            break;
        }
    }

    let struct_name = input_struct.ident;
    let str_path = match args.str_path {
        Some(v) => v,
        None => panic!("Attribute `path` not set, required"),
    };
    let request_method = args.request_method;
    let required_permission = args.required_permission;
    let input_struct_name = args.input_struct_name;
    let output_struct_name = args.output_struct_name;
    let scope_struct_name = args.scope_struct_name;

    quote! {
        #[automatically_derived]
        impl crate::endpoint::Endpoint for #struct_name {
            const PATH: &'static str = #str_path;
            const REQUEST_METHOD: RequestMethod = #request_method;
            const REQUIRED_PERMISSION: RequiredPermission = #required_permission;

            type InputStruct = #input_struct_name;
            type OutputStruct = #output_struct_name;

            type ScopeStruct = #scope_struct_name;
        }
    }
    .into()
}

#[proc_macro_derive(FromIntoInner)]
pub fn derive_frominto_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as syn::ItemStruct);

    if input_struct.fields.len() != 1 {
        panic!("Struct Length != 1");
    }

    let field = input_struct.fields.iter().next().unwrap();

    if field.ident.is_some() {
        panic!("Not tuple struct");
    }

    let struct_name = input_struct.ident;
    let field_type = &field.ty;

    quote! {
        #[automatically_derived]
        impl From<#field_type> for #struct_name {
            fn from(value: #field_type) -> Self {
                Self(value)
            }
        }

        #[automatically_derived]
        impl From<#struct_name> for #field_type {
            fn from(value: #struct_name) -> Self {
                value.0
            }
        }
    }
    .into()
}

#[proc_macro_derive(InputFromActix)]
pub fn derive_input_from_actix(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as syn::ItemStruct);
    let struct_name = input_struct.ident;

    let mut return_data = match input_struct.fields{
        syn::Fields::Unit => return quote! {
            impl crate::compatibility_layer::rust_actix::from_input::InputFromActix for #struct_name {
                fn get_from_request(request: &mut actix_web::HttpRequest) -> Result<Self, crate::error::FinalErrorResponse> {
                    Ok(Self)
                }
            }
        }.into(),
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) => proc_macro2::TokenStream::new(),
    };

    let mut token_data_tuple = proc_macro2::TokenStream::new();
    let mut inner_match = proc_macro2::TokenStream::new();

    for (field_num, field) in input_struct.fields.iter().enumerate() {
        let mut args = None;
        for attr in &field.attrs {
            if utils::attribute_is_internal(attr) {
                args = Some(attr.parse_args().expect("Couldn't parse args"));
                break;
            }
        }
        let args: QueryArgs = args.unwrap_or_default();
        
        if !args.query_keys.is_empty()
        quote! { None, }.to_tokens(&mut token_data_tuple);


        for (i, key) in args.query_keys.iter().enumerate() {
            quote! { Some(#key) }.to_tokens(&mut inner_match);
            if i != 0 {
                quote! { | }.to_tokens(&mut inner_match);
            }
        }

                quote! { => acc.#field_num = split.next() }.to_tokens(&mut inner_match);
                if let Some(v) = args.map {
                quote! { .map(#v) }.to_tokens(&mut inner_match);
                }
                quote! { , }.to_tokens(&mut inner_match);
        }
    }

    let return_data_parenthesized = match input_struct.fields {
        syn::Fields::Unit => unsafe { unreachable_unchecked() },
        syn::Fields::Named(_) => quote! { { #return_data } }, syn::Fields::Unnamed(_) => quote! { ( #return_data ) },
    };

    quote! {
        impl crate::compatibility_layer::rust_actix::from_input::InputFromActix for #struct_name {
            fn get_from_request(request: &mut actix_web::HttpRequest) -> Result<Self, crate::error::FinalErrorResponse> {
                let data = request.query_string().split(&['?', '&']).fold(
                    ( #token_data_tuple ),
                    |mut acc, next| {
                        let mut split = next.split('=');
                        match split.next() {
                            #inner_match
                            _ => (),
                        };
                        acc
                    },
                );

                Ok(Self #return_data_parenthesized )
            };
        }
    }.into()
}

#[proc_macro]
pub fn to_scope(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let v = parse_macro_input!(input as to_scope::Scope);
    v.into_token_stream().into()
}
