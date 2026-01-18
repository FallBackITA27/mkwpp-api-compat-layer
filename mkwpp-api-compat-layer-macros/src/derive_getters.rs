use proc_macro2::Span;
use quote::quote;
use syn::{Ident, parse_macro_input};

pub fn derive_getter(
    input: proc_macro::TokenStream,
    attribute_name: &str,
    get_trait_name: Ident,
    has_trait_name: Ident,
    get_function_name: Ident,
    out_type: syn::Type,
) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as syn::ItemStruct);

    let mut out_ident = None;

    let mut get_punctuated = syn::punctuated::Punctuated::new();
    get_punctuated.push_value(syn::PathSegment {
        ident: Ident::new("crate", Span::call_site()),
        arguments: syn::PathArguments::None,
    });
    get_punctuated.push_punct(syn::token::PathSep::default());
    get_punctuated.push_value(syn::PathSegment {
        ident: Ident::new("common_data_traits", Span::call_site()),
        arguments: syn::PathArguments::None,
    });
    get_punctuated.push_punct(syn::token::PathSep::default());
    let mut has_punctuated = get_punctuated.clone();
    get_punctuated.push_value(syn::PathSegment {
        ident: get_trait_name,
        arguments: syn::PathArguments::None,
    });
    has_punctuated.push_value(syn::PathSegment {
        ident: has_trait_name,
        arguments: syn::PathArguments::None,
    });
    let get_trait_path = syn::Path {
        leading_colon: None,
        segments: get_punctuated,
    };
    let has_trait_path = syn::Path {
        leading_colon: None,
        segments: has_punctuated,
    };

    for (field_number, field) in input_struct.fields.iter().enumerate() {
        for attr in field.attrs.iter() {
            let ident = match attr.path().get_ident() {
                Some(v) => v,
                None => continue,
            };
            if ident.to_string().as_str() != "internal" {
                continue;
            }

            let possible_out_ident = match &field.ident {
                Some(v) => v,
                None => &Ident::new(field_number.to_string().as_str(), Span::call_site()),
            };

            if let Err(v) = attr.parse_nested_meta(|meta| {
                let ident = match meta.path.get_ident() {
                    Some(v) => v,
                    None => {
                        return syn::Result::Err(syn::Error::new(
                            Span::call_site(),
                            "Invalid internal value",
                        ));
                    }
                };

                if ident.to_string().as_str() == attribute_name {
                    out_ident = Some(possible_out_ident.clone());
                }
                Ok(())
            }) {
                panic!("{v}");
            }
        }
    }

    let struct_name = input_struct.ident;

    match out_ident {
        None => quote! { #[automatically_derived] impl #get_trait_path for #struct_name {} },
        Some(v) => quote! {
            #[automatically_derived]
            impl #has_trait_path for #struct_name {
                #[inline]
                fn #get_function_name(&self) -> #out_type {
                    self.#v
                }
            }
        },
    }
    .into()
}
