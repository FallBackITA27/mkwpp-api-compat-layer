use crate::{
    common_data_traits::{GetCategory, GetId, GetSessionToken, HasId},
    common_types::Filter,
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
    type OutputStruct = Vec<GetBlogList>;

    type ScopeStruct = BlogScope;
}

struct GetBlogListInput {
    filter: Filter,
}
impl GetId for GetBlogListInput {}
impl GetCategory for GetBlogListInput {}
impl GetSessionToken for GetBlogListInput {}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetBlogPost;

impl Endpoint for GetBlogPost {
    const PATH: &'static str = "/get_post";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = GetBlogListInput;
    type OutputStruct = Vec<GetBlogList>;

    type ScopeStruct = BlogScope;
}

struct GetBlogPostInput {
    id: i32,
}
impl HasId for GetBlogPostInput {
    fn get_id(&self) -> i32 {
        self.id
    }
}
impl GetCategory for GetBlogPostInput {}
impl GetSessionToken for GetBlogPostInput {}
