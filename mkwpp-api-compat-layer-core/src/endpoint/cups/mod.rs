use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    endpoint::{RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub struct CupsScope;

impl Scope for CupsScope {
    const PATH: &'static str = "/cups";

    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get", output = Vec<GetCupsOutput>, scope = CupsScope)]
pub struct GetCups;

#[derive(serde::Deserialize, GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
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

#[derive(serde::Deserialize, GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(Clone, Copy))]
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
