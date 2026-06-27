use mkwpp_api_compat_layer_macros::{
    Endpoint, GetCategory, GetId, GetSessionToken,
};

use crate::{
    common_data_traits::GetCategory,
    common_types::{UtcTimestamp, category::Category},
    endpoint::{Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct ChampsScope;
impl Scope for ChampsScope {
    const PATH: &'static str = "/champs";
    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get", input = GetChampsFilters, output = Vec<GetChampsOutput>, scope = ChampsScope)]
pub struct GetChamps;

#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(serde::Deserialize, GetId, GetSessionToken)]
pub struct GetChampsFilters {
    #[wasm_bindgen(readonly)]
    pub(crate) category: Option<Category>,
}

impl GetCategory for GetChampsFilters {
    fn get_category(&self) -> Category {
        self.category.unwrap_or(Category::Normal)
    }
}

#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
#[derive(GetId, GetCategory, GetSessionToken)]
pub struct GetChampsOutput {
    #[wasm_bindgen(readonly)]
    #[internal(id)]
    pub id: i32,

    #[wasm_bindgen(readonly)]
    pub player_id: i32,

    #[wasm_bindgen(readonly)]
    #[internal(category)]
    pub category: Category,

    #[wasm_bindgen(readonly)]
    pub date_instated: UtcTimestamp,
}
