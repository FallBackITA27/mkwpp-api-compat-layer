use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    endpoint::{Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct StandardLevelsScope;
impl Scope for StandardLevelsScope {
    const PATH: &'static str = "/standard_levels";
    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get", output = Vec<GetStandardLevelsOutput>, scope = StandardLevelsScope)]
pub struct GetStandardLevels;

#[derive(GetId, GetCategory, GetSessionToken)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct GetStandardLevelsOutput {
    #[wasm_bindgen(readonly)]
    #[internal(id)]
    pub id: i32,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub code: String,

    #[wasm_bindgen(readonly)]
    pub value: i32,

    #[wasm_bindgen(readonly)]
    pub is_legacy: bool,
}
