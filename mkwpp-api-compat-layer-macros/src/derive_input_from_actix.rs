use proc_macro2::Span;
use syn::{Ident, Token, parse::Parse};

use crate::internal::{FieldDerived, FieldKey, FieldRequired, IsInternalAttribute};

pub struct InputFromActixArgs {
    field_args: Vec<InputFromActixFieldArgs>
}

pub struct InputFromActixFieldArgs {
    pub query_keys: Vec<FieldKey>,
    pub required: FieldRequired,
    pub derived: FieldDerived,
}

impl Default for InputFromActixFieldArgs {
    fn default() -> Self {
        Self { query_keys: vec![], required: FieldRequired(syn::LitBool::new(false, Span::call_site())), derived: FieldDerived(syn::LitBool::new(true, Span::call_site()))}
    }
}

impl Parse for InputFromActixFieldArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = InputFromActixFieldArgs::default();
        loop {
            if input.is_empty() {
                break;
            }

            let key = input.parse::<Ident>()?.to_string();

            match key.as_str() {
                FieldRequired::NAME => out.required = input.parse()?,
                FieldKey::NAME => out.query_keys.push(input.parse()?),
                FieldDerived::NAME => out.derived = input.parse()?,
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
