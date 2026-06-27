use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Ident, Token, bracketed, parse::Parse};

pub fn camel_case_to_snake_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut iterator = input.char_indices();
    let mut out_name = String::from(iterator.next().unwrap().1.to_ascii_lowercase());
    let mut last_idx = 1;

    for (idx, char) in iterator {
        if char.is_uppercase() {
            out_name += &input[last_idx..idx];
            out_name.push('_');
            out_name.push(char.to_ascii_lowercase());
            last_idx = idx + 1;
        }
    }

    out_name += &input[last_idx..];

    out_name
}

pub struct Scope {
    scope_name: Ident,
    child_endpoints: Vec<Ident>,
    child_scopes: Vec<Scope>,
}

impl Parse for Scope {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = Self {
            scope_name: input.parse()?,
            child_scopes: vec![],
            child_endpoints: vec![],
        };

        let _out_most = input.parse::<Token![:]>()?;
        let field_list;
        bracketed!(field_list in input);
        let input = field_list;

        while !input.is_empty() {
            if input.peek2(Token![:]) {
                out.child_scopes.push(Parse::parse(&input)?);
            } else {
                out.child_endpoints.push(input.parse()?);
            }

            match input.parse::<Token![,]>() {
                _ if input.is_empty() => break,
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(out)
    }
}

impl ToTokens for Scope {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let inputs = self.get_inputs_token_stream();
        let where_clauses = self.get_where_clause_token_stream();
        let routes = self.get_actix_scope_token_stream();

        let scope_name = &self.scope_name;
        let final_out = quote! {
            impl #scope_name {
                pub fn to_actix_scope( #inputs ) -> actix_web::Scope
                    where #where_clauses
                { #routes }
            }
        };

        tokens.extend(final_out);
    }
}

impl Scope {
    fn get_inputs_token_stream(&self) -> proc_macro2::TokenStream {
        let mut inputs = proc_macro2::TokenStream::new();
        for endpoint in &self.child_endpoints {
            let new_ident = Ident::new(
                (camel_case_to_snake_case(endpoint.to_string().as_str()) + "_handler").as_str(),
                Span::call_site(),
            );
            inputs.extend(quote! {
                #new_ident: impl AsyncFn(<#endpoint as Endpoint>::InputStruct)
                    -> PPResult<<#endpoint as Endpoint>::OutputStruct> + 'static,
            });
        }

        for scope in &self.child_scopes {
            inputs.extend(scope.get_inputs_token_stream());
        }

        inputs
    }

    fn get_where_clause_token_stream(&self) -> proc_macro2::TokenStream {
        let mut where_clauses = proc_macro2::TokenStream::new();

        for endpoint in &self.child_endpoints {
            where_clauses.extend(quote! {
                <#endpoint as Endpoint>::InputStruct: serde::de::DeserializeOwned,
                <#endpoint as Endpoint>::OutputStruct: serde::Serialize,
            });
        }

        for scope in &self.child_scopes {
            where_clauses.extend(scope.get_where_clause_token_stream());
        }

        where_clauses
    }

    fn get_actix_scope_token_stream(&self) -> proc_macro2::TokenStream {
        let scope_name = &self.scope_name;
        let mut scopes = quote! { web::scope(#scope_name::PATH) };

        for endpoint in &self.child_endpoints {
            let new_ident = Ident::new(
                (camel_case_to_snake_case(endpoint.to_string().as_str()) + "_handler").as_str(),
                Span::call_site(),
            );
            scopes.extend(quote! {
                .route(#endpoint::PATH, #endpoint::to_actix_route(#new_ident))
            });
        }

        for scope in &self.child_scopes {
            let v = scope.get_actix_scope_token_stream();
            scopes.extend(quote! {
                .service(#v)
            });
        }

        scopes
    }
}
