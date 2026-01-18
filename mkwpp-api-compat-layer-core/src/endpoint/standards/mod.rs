use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};

use crate::{
    common_data_traits::HasId,
    common_types::{Category, NoData},
    endpoint::{Endpoint, Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct StandardsScope;
impl Scope for StandardsScope {
    const PATH: &'static str = "/standards";
    type OuterScope = Root;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetStandards;

impl Endpoint for GetStandards {
    const PATH: &'static str = "/get";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = Vec<GetStandardsOutput>;

    type ScopeStruct = StandardsScope;
}

#[derive(GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct GetStandardsOutput {
    #[wasm_bindgen(readonly)]
    #[internal(id)]
    pub id: i32,

    #[wasm_bindgen(readonly)]
    pub standard_level_id: i32,

    #[wasm_bindgen(readonly)]
    pub track_id: i32,

    #[wasm_bindgen(readonly)]
    pub category: Category,

    #[wasm_bindgen(readonly)]
    pub is_lap: bool,

    #[wasm_bindgen(readonly)]
    pub value: Option<i32>,
}
