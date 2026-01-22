use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken, InputFromActix};

use crate::{
    common_types::{UtcTimestamp, limit::Limit},
    endpoint::{Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct BlogScope;
impl Scope for BlogScope {
    const PATH: &'static str = "/blog";
    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_list", input = GetBlogListInput, output = Vec<BlogPost>, scope = BlogScope)]
pub struct GetBlogList;

#[derive(GetId, GetCategory, GetSessionToken, InputFromActix)]
pub struct GetBlogListInput {
    filter: Limit,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_post", input = GetBlogPostInput, output = BlogPost, scope = BlogScope)]
pub struct GetBlogPost;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct GetBlogPostInput {
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
