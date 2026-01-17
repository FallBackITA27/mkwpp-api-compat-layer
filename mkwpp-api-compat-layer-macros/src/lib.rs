use syn::{Ident, parse_macro_input};

#[proc_macro_derive(GetId)]
pub fn derive_get_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as syn::ItemStruct);

    let id_field_ident = None;
    for (field_number, field) in input_struct.fields.iter().enumerate() {
        for attr in field.attrs {
            let ident = match attr.path().get_ident() {
                Some(v) => v,
                None => continue,
            };
            if ident.to_string().as_str() != "internal" {
                continue;
            }

            if let Err(v) = attr.parse_nested_meta(|meta| {
                
                Ok(())
            }) {
                panic!("{v}");
            }
        }
    }

    let mut out = input_struct.to_token_stream();

    let out_impl = syn::ItemImpl {};
}
