use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{Category, NoData},
    endpoint::{Endpoint, RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub struct TracksScope;

impl Scope for TracksScope {
    const PATH: &'static str = "/tracks";

    type OuterScope = Root;
}

#[derive(Default)]
pub struct GetTracks;

impl Endpoint for GetTracks {
    const PATH: &'static str = "/get";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = Vec<GetTracksOutput>;

    type ScopeStruct = TracksScope;
}

#[derive(GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct GetTracksOutput {
    #[wasm_bindgen(readonly)]
    #[internal(id)]
    pub id: i32,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub abbr: String,

    #[wasm_bindgen(readonly)]
    pub cup_id: i32,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub categories: Vec<Category>,
}
