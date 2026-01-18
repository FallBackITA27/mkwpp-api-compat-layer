use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::NoData,
    endpoint::{Endpoint, RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub struct CupsScope;

impl Scope for CupsScope {
    const PATH: &'static str = "/cups";

    type OuterScope = Root;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetCups;

impl Endpoint for GetCups {
    const PATH: &'static str = "/get";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = Vec<GetCupsOutput>;

    type ScopeStruct = CupsScope;
}

#[derive(GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct GetCupsOutput {
    #[cfg_attr(feature = "typescript-wasm", wasm_bindgen(readonly))]
    #[internal(id)]
    pub id: i32,

    #[cfg_attr(feature = "typescript-wasm", wasm_bindgen(getter_with_clone))]
    #[cfg_attr(feature = "typescript-wasm", wasm_bindgen(readonly))]
    pub code: String,

    #[cfg_attr(feature = "typescript-wasm", wasm_bindgen(readonly))]
    pub track_ids: CupSlots,
}

#[derive(GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize, Clone, Copy))]
pub struct CupSlots {
    #[cfg_attr(feature = "typescript-wasm", wasm_bindgen(readonly))]
    pub track_id_slot_1: i32,

    #[cfg_attr(feature = "typescript-wasm", wasm_bindgen(readonly))]
    pub track_id_slot_2: i32,

    #[cfg_attr(feature = "typescript-wasm", wasm_bindgen(readonly))]
    pub track_id_slot_3: i32,

    #[cfg_attr(feature = "typescript-wasm", wasm_bindgen(readonly))]
    pub track_id_slot_4: i32,
}

impl From<[i32; 4]> for CupSlots {
    fn from(value: [i32; 4]) -> Self {
        let [
            track_id_slot_1,
            track_id_slot_2,
            track_id_slot_3,
            track_id_slot_4,
        ] = value;
        Self {
            track_id_slot_1,
            track_id_slot_2,
            track_id_slot_3,
            track_id_slot_4,
        }
    }
}
