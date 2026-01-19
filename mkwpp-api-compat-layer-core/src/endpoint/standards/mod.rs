use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::category::Category,
    endpoint::{Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct StandardsScope;
impl Scope for StandardsScope {
    const PATH: &'static str = "/standards";
    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get", output = Vec<GetStandardsOutput>, scope = StandardsScope)]
pub struct GetStandards;

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
