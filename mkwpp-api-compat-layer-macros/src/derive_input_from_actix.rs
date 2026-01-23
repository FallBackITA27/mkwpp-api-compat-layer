use syn::{Ident, Token, parse::Parse};

#[derive(Default)]
pub struct QueryArgs {
    pub query_keys: Vec<syn::LitStr>,
    pub query_map: Option<syn::Expr>,
}

impl Parse for QueryArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = QueryArgs::default();
        loop {
            if input.is_empty() {
                break;
            }

            let key = input.parse::<Ident>()?.to_string();

            let _ = input.parse::<Token![=]>()?;

            match key.as_str() {
                "key" => out.query_keys.push(input.parse()?),
                "map" => out.query_map = Some(input.parse()?),
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
