use proc_macro2::Span;
use syn::{Ident, Token, parse::Parse};

use crate::internal::{FieldKey, FieldRequired, IsInternalAttribute};

pub struct ArgumentGetter {
    pub query_keys: Vec<FieldKey>,
    pub required: FieldRequired,
}

impl Default for ArgumentGetter {
    fn default() -> Self {
        Self { query_keys: vec![], required: FieldRequired(syn::LitBool::new(false, Span::call_site())) }
    }
}

impl Parse for ArgumentGetter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = ArgumentGetter::default();
        loop {
            if input.is_empty() {
                break;
            }

            let key = input.parse::<Ident>()?.to_string();

            match key.as_str() {
                FieldRequired::NAME => out.required = input.parse()?,
                FieldKey::NAME => out.query_keys.push(input.parse()?),
                _ => (),
            }

            if input.is_empty() {
                break;
            }

            let _ = input.parse::<Token![,]>()?;
        }

        Ok(out)
    }
}
