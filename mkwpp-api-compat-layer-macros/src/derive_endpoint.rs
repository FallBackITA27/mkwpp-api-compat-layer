use quote::quote;
use syn::{Ident, Token, parse::Parse};

pub struct EndpointArgs {
    pub str_path: Option<syn::LitStr>,
    pub input_struct_name: syn::Type,
    pub output_struct_name: syn::Type,
    pub scope_struct_name: syn::Type,
    pub required_permission: syn::Expr,
    pub request_method: syn::Expr,
}

impl Default for EndpointArgs {
    fn default() -> Self {
        Self {
            str_path: None,
            input_struct_name: syn::Type::Verbatim(quote! { crate::common_types::NoData }),
            output_struct_name: syn::Type::Verbatim(quote! { crate::common_types::NoData }),
            scope_struct_name: syn::Type::Verbatim(quote! { crate::endpoint::Root }),
            required_permission: syn::Expr::Verbatim(
                quote! { crate::required_permission::RequiredPermission::None },
            ),
            request_method: syn::Expr::Verbatim(
                quote! { crate::request_method::RequestMethod::Get },
            ),
        }
    }
}

impl Parse for EndpointArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = EndpointArgs::default();
        loop {
            if input.is_empty() {
                break;
            }

            let key = input.parse::<Ident>()?.to_string();

            let _ = input.parse::<Token![=]>()?;

            match key.as_str() {
                "path" => out.str_path = input.parse()?,
                "input" => out.input_struct_name = input.parse()?,
                "output" => out.output_struct_name = input.parse()?,
                "scope" => out.scope_struct_name = input.parse()?,
                "required" => out.required_permission = input.parse()?,
                "request" => out.request_method = input.parse()?,
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
