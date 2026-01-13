use crate::{
    common_data_traits::HasId,
    common_types::{Category, NoData},
    endpoint::{Endpoint, Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct StandardLevelsScope;
impl Scope for StandardLevelsScope {
    const PATH: &'static str = "/standard_levels";
    type OuterScope = Root;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetStandardLevels;

impl Endpoint for GetStandardLevels {
    const PATH: &'static str = "/get";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = Vec<GetStandardLevelsOutput>;

    type ScopeStruct = StandardLevelsScope;
}

#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct GetStandardLevelsOutput {
    #[wasm_bindgen(readonly)]
    pub id: i32,
    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub code: String,
    #[wasm_bindgen(readonly)]
    pub value: i32,
    #[wasm_bindgen(readonly)]
    pub is_legacy: bool,
}

impl HasId for GetStandardLevelsOutput {
    fn get_id(&self) -> i32 {
        self.id
    }
}
