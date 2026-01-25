use quote::quote;
use syn::{Ident, Token, parse::Parse};

use crate::internal::{EndpointInput, EndpointOutput, EndpointPath, EndpointRequestMethod, EndpointRequiredPermissions, EndpointScope, IsInternalAttribute};

pub struct EndpointArgs {
    pub str_path: Option<EndpointPath>,
    pub input_struct_name: EndpointInput,
    pub output_struct_name: EndpointOutput,
    pub scope_struct_name: EndpointScope,
    pub required_permission: EndpointRequiredPermissions,
    pub request_method: EndpointRequestMethod
}

impl Default for EndpointArgs {
    fn default() -> Self {
        Self {
            str_path: None,
            input_struct_name: EndpointInput(syn::Type::Verbatim(quote! { crate::common_types::NoData })),
            output_struct_name:EndpointOutput(syn::Type::Verbatim(quote! { crate::common_types::NoData })),
            scope_struct_name:EndpointScope( syn::Type::Verbatim(quote! { crate::endpoint::Root })),
            required_permission:EndpointRequiredPermissions( syn::Expr::Verbatim(
                quote! { crate::required_permission::RequiredPermission::None },
            )),
            request_method:EndpointRequestMethod( syn::Expr::Verbatim(
                quote! { crate::request_method::RequestMethod::Get },
            )),
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
            
            match key.as_str() {
                EndpointPath::NAME => out.str_path = Some(input.parse::<EndpointPath>()?),
                EndpointInput::NAME => out.input_struct_name = input.parse()?,
                EndpointOutput::NAME => out.output_struct_name = input.parse()?,
                EndpointScope::NAME => out.scope_struct_name = input.parse()?,
                EndpointRequiredPermissions::NAME => out.required_permission = input.parse()?,
                EndpointRequestMethod::NAME => out.request_method = input.parse()?,
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
