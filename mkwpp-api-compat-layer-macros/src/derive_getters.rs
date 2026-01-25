use proc_macro2::Span;
use quote::quote;
use syn::{Ident, parse_macro_input};

use crate::internal::{IsInternalAttribute, attribute_is_match};

pub fn derive_getter<T: IsInternalAttribute<InnerType = syn::LitBool>>(
    input: proc_macro::TokenStream,
    get_trait_name: Ident,
    has_trait_name: Ident,
    get_function_name: Ident,
    out_type: syn::Type,
    return_reference: bool,
) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as syn::ItemStruct);

    let mut out_ident = None;

    let get_trait_path = quote! { crate::common_data_traits::#get_trait_name };
    let has_trait_path = quote! { crate::common_data_traits::#has_trait_name };

    for (field_number, field) in input_struct.fields.iter().enumerate() {
        for attr in field.attrs.iter() {
            if T::attribute_is_match_deep(attr)
                && attr
                    .parse_args::<T>()
                    .expect("Failed to parse syntax")
                    .get_inner()
                    .value
            {
                out_ident = Some(match &field.ident {
                    Some(v) => Some(v.clone()),
                    None => Some(Ident::new(
                        field_number.to_string().as_str(),
                        Span::call_site(),
                    )),
                });
            }
        }
    }

    let struct_name = input_struct.ident;

    match out_ident {
        None => quote! { #[automatically_derived] impl #get_trait_path for #struct_name {} },
        Some(v) if return_reference => quote! {
            #[automatically_derived]
            impl #has_trait_path for #struct_name {
                #[inline]
                fn #get_function_name(&self) -> #out_type {
                    &self.#v
                }
            }
        },
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
