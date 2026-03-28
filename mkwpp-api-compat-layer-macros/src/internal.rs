use proc_macro2::Span;
use syn::{Token, parse::Parse};

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

#[inline(always)]
fn parse_internal_attribute_in_loop<T: IsInternalAttribute<InnerType = syn::LitBool>>(
    input: syn::parse::ParseStream,
) -> syn::Result<T> {
    match T::OPT_EQ_SIGN {
        true => match input.peek(Token![=]) {
            false => Ok(T::from(syn::LitBool::new(true, Span::call_site()))),
            true => {
                let _: Token![=] = input.parse()?;
                input.parse::<T::InnerType>().map(Into::into)
            }
        },
        false => parse_internal_attribute_in_loop_not_toggle(input),
    }
}

#[inline(always)]
fn parse_internal_attribute_in_loop_not_toggle<T: IsInternalAttribute>(
    input: syn::parse::ParseStream,
) -> syn::Result<T> {
    let _: Token![=] = input.parse()?;
    input.parse::<T::InnerType>().map(Into::into)
}

pub trait IsInternalAttribute: Parse + From<Self::InnerType> {
    const NAME: &'static str;
    const OPT_EQ_SIGN: bool;

    type InnerType: Parse;

    fn get_inner(self) -> Self::InnerType;

    fn get_attribute_name(&self) -> &'static str {
        Self::NAME
    }

    fn attribute_is_match_shallow(path: &syn::Path) -> bool {
        let ident = match path.get_ident() {
            Some(v) => v,
            None => {
                panic!("Invalid internal value",);
            }
        };
        ident.to_string().as_str() == Self::NAME
    }

    fn attribute_is_match_deep(attr: &syn::Attribute) -> bool {
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

            if ident.to_string().as_str() == Self::NAME {
                out = true;
            }

            Ok(())
        }) {
            panic!("{v}");
        }

        out
    }
}

pub struct EndpointPath(pub syn::LitStr);
impl IsInternalAttribute for EndpointPath {
    const NAME: &'static str = "path";
    const OPT_EQ_SIGN: bool = false;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::LitStr;
}
impl Parse for EndpointPath {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop_not_toggle(input)
    }
}
impl From<syn::LitStr> for EndpointPath {
    fn from(value: syn::LitStr) -> Self {
        Self(value)
    }
}

pub struct EndpointInput(pub syn::Type);
impl IsInternalAttribute for EndpointInput {
    const NAME: &'static str = "input";
    const OPT_EQ_SIGN: bool = false;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::Type;
}
impl Parse for EndpointInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop_not_toggle(input)
    }
}
impl From<syn::Type> for EndpointInput {
    fn from(value: syn::Type) -> Self {
        Self(value)
    }
}

pub struct EndpointOutput(pub syn::Type);
impl IsInternalAttribute for EndpointOutput {
    const NAME: &'static str = "output";
    const OPT_EQ_SIGN: bool = false;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::Type;
}
impl Parse for EndpointOutput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop_not_toggle(input)
    }
}
impl From<syn::Type> for EndpointOutput {
    fn from(value: syn::Type) -> Self {
        Self(value)
    }
}

pub struct EndpointScope(pub syn::Type);
impl IsInternalAttribute for EndpointScope {
    const NAME: &'static str = "scope";
    const OPT_EQ_SIGN: bool = false;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::Type;
}
impl Parse for EndpointScope {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop_not_toggle(input)
    }
}
impl From<syn::Type> for EndpointScope {
    fn from(value: syn::Type) -> Self {
        Self(value)
    }
}

pub struct EndpointRequiredPermissions(pub syn::Expr);
impl IsInternalAttribute for EndpointRequiredPermissions {
    const NAME: &'static str = "perms";
    const OPT_EQ_SIGN: bool = false;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::Expr;
}
impl Parse for EndpointRequiredPermissions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop_not_toggle(input)
    }
}
impl From<syn::Expr> for EndpointRequiredPermissions {
    fn from(value: syn::Expr) -> Self {
        Self(value)
    }
}

pub struct EndpointRequestMethod(pub syn::Expr);
impl IsInternalAttribute for EndpointRequestMethod {
    const NAME: &'static str = "request_method";
    const OPT_EQ_SIGN: bool = false;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::Expr;
}
impl Parse for EndpointRequestMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop_not_toggle(input)
    }
}
impl From<syn::Expr> for EndpointRequestMethod {
    fn from(value: syn::Expr) -> Self {
        Self(value)
    }
}

pub struct FieldIsId(pub syn::LitBool);
impl IsInternalAttribute for FieldIsId {
    const NAME: &'static str = "id";
    const OPT_EQ_SIGN: bool = true;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::LitBool;
}
impl Parse for FieldIsId {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop(input)
    }
}
impl From<syn::LitBool> for FieldIsId {
    fn from(value: syn::LitBool) -> Self {
        Self(value)
    }
}

pub struct FieldIsCategory(pub syn::LitBool);
impl IsInternalAttribute for FieldIsCategory {
    const NAME: &'static str = "category";
    const OPT_EQ_SIGN: bool = true;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::LitBool;
}
impl Parse for FieldIsCategory {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop(input)
    }
}
impl From<syn::LitBool> for FieldIsCategory {
    fn from(value: syn::LitBool) -> Self {
        Self(value)
    }
}

pub struct FieldIsSessionToken(pub syn::LitBool);
impl IsInternalAttribute for FieldIsSessionToken {
    const NAME: &'static str = "session_token";
    const OPT_EQ_SIGN: bool = true;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::LitBool;
}
impl Parse for FieldIsSessionToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop(input)
    }
}
impl From<syn::LitBool> for FieldIsSessionToken {
    fn from(value: syn::LitBool) -> Self {
        Self(value)
    }
}

pub struct FieldDerived(pub syn::LitBool);
impl IsInternalAttribute for FieldDerived {
    const NAME: &'static str = "derived";
    const OPT_EQ_SIGN: bool = true;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::LitBool;
}
impl Parse for FieldDerived {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop(input)
    }
}
impl From<syn::LitBool> for FieldDerived {
    fn from(value: syn::LitBool) -> Self {
        Self(value)
    }
}


pub struct FieldRequired(pub syn::LitBool);
impl IsInternalAttribute for FieldRequired {
    const NAME: &'static str = "required";
    const OPT_EQ_SIGN: bool = true;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::LitBool;
}
impl Parse for FieldRequired {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop(input)
    }
}
impl From<syn::LitBool> for FieldRequired {
    fn from(value: syn::LitBool) -> Self {
        Self(value)
    }
}

pub struct FieldDescription(pub syn::LitStr);
impl IsInternalAttribute for FieldDescription {
    const NAME: &'static str = "description";
    const OPT_EQ_SIGN: bool = true;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::LitStr;
}
impl Parse for FieldDescription {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop_not_toggle(input)
    }
}
impl From<syn::LitStr> for FieldDescription {
    fn from(value: syn::LitStr) -> Self {
        Self(value)
    }
}

pub struct FieldKey(pub syn::LitStr);
impl IsInternalAttribute for FieldKey {
    const NAME: &'static str = "key";
    const OPT_EQ_SIGN: bool = true;

    fn get_inner(self) -> Self::InnerType {
        self.0
    }

    type InnerType = syn::LitStr;
}
impl Parse for FieldKey {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_internal_attribute_in_loop_not_toggle(input)
    }
}
impl From<syn::LitStr> for FieldKey {
    fn from(value: syn::LitStr) -> Self {
        Self(value)
    }
}
