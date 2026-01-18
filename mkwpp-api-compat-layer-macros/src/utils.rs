pub fn attribute_is_internal(attr: &syn::Attribute) -> bool {
    let ident = match attr.path().get_ident() {
        Some(v) => v,
        None => return false,
    };

    ident.to_string().as_str() == "internal"
}

pub fn attribute_is_match(attr: &syn::Attribute, str_to_match: &str) -> bool {
    if !attribute_is_internal(attr) {
        return false;
    }

    let mut out = false;

    if let Err(v) = attr.parse_nested_meta(|meta| {
        let ident = match meta.path.get_ident() {
            Some(v) => v,
            None => {
                return syn::Result::Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "Invalid internal value",
                ));
            }
        };

        if ident.to_string().as_str() == str_to_match {
            out = true;
        }

        Ok(())
    }) {
        panic!("{v}");
    }

    out
}

