use syn::{Ident, Token, parse::Parse};

#[derive(Default)]
pub struct ArgumentGetter {
    pub query_keys: Vec<syn::LitStr>,
    pub query_map: Option<syn::Expr>,
    pub derive: bool
}

impl Parse for ArgumentGetter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = ArgumentGetter::default();
        loop {
            if input.is_empty() {
                break;
            }

            let key = input.parse::<Ident>()?.to_string();

            let _ = input.parse::<Token![=]>()?;

            match key.as_str() {
                "derive" => out.derive = input.parse::<syn::LitBool>()?.value,
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
