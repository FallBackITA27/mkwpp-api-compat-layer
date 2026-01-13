use crate::{
    common_data_traits::{GetCategory, GetId, GetSessionToken, HasId},
    common_types::{Category, NoData, UtcTimestamp},
    endpoint::{Endpoint, Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct ChampsScope;
impl Scope for ChampsScope {
    const PATH: &'static str = "/champs";
    type OuterScope = Root;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetChamps;

impl Endpoint for GetChamps {
    const PATH: &'static str = "/get";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = GetChampsFilters;
    type OutputStruct = Vec<GetChampsOutput>;

    type ScopeStruct = ChampsScope;
}

#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetChampsFilters {
    #[wasm_bindgen(readonly)]
    pub(crate) category: Option<Category>,
}

impl GetId for GetChampsFilters {}
impl GetSessionToken for GetChampsFilters {}
impl GetCategory for GetChampsFilters {
    fn get_category(&self) -> Category {
        self.category.unwrap_or(Category::Normal)
    }
}

#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct GetChampsOutput {
    #[wasm_bindgen(readonly)]
    pub id: i32,
    #[wasm_bindgen(readonly)]
    pub player_id: i32,
    #[wasm_bindgen(readonly)]
    pub category: Category,
    #[wasm_bindgen(readonly)]
    pub date_instated: UtcTimestamp,
}

impl HasId for GetChampsOutput {
    fn get_id(&self) -> i32 {
        self.id
    }
}
