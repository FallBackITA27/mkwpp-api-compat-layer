use proc_macro2::Span;
use quote::quote;
use syn::Ident;

mod derive_getters;

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

