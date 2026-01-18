use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};

use crate::{
    common_data_traits::{GetCategory, GetSessionToken},
    common_types::{Filter, UtcTimestamp},
    endpoint::{Endpoint, Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct BlogScope;
impl Scope for BlogScope {
    const PATH: &'static str = "/blog";
    type OuterScope = Root;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetBlogList;

impl Endpoint for GetBlogList {
    const PATH: &'static str = "/get_list";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = GetBlogListInput;
    type OutputStruct = Vec<BlogPost>;

    type ScopeStruct = BlogScope;
}

#[derive(GetId, GetCategory, GetSessionToken)]
struct GetBlogListInput {
    filter: Filter,
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetBlogPost;

impl Endpoint for GetBlogPost {
    const PATH: &'static str = "/get_post";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = GetBlogListInput;
    type OutputStruct = Vec<BlogPost>;

    type ScopeStruct = BlogScope;
}

#[derive(GetId, GetCategory, GetSessionToken)]
struct GetBlogPostInput {
    #[internal(id)]
    id: i32,
}

#[derive(Debug, Clone, GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct BlogPost {
    #[wasm_bindgen(readonly)]
    #[internal(id)]
    pub id: i32,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub title: String,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub content: String,

    #[wasm_bindgen(readonly)]
    pub is_published: bool,

    #[wasm_bindgen(readonly)]
    pub published_at: UtcTimestamp,

    #[wasm_bindgen(readonly)]
    pub author_id: Option<i32>,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub username: Option<String>,
}
