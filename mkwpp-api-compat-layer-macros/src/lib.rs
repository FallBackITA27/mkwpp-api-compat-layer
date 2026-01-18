use proc_macro2::Span;
use quote::quote;
use syn::{Ident, parse_macro_input};

use crate::derive_endpoint::EndpointArgs;

mod derive_endpoint;
mod derive_getters;
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
        syn::Type::Verbatim(quote! { crate::common_types::Category }),
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
