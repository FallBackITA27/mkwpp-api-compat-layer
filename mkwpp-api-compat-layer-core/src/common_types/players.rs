use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};

use crate::common_types::{ChadsoftID, UtcTimestamp};

#[either_field::make_template(
    GenStructs: true,
    DeleteTemplate: true,
    OmitEmptyTupleFields: true;
    pub PlayersLoggedIn: [
        bio: Option<String>,
        pronouns: Option<String>,
        region_id: i32,
        joined_date: UtcTimestamp,
        last_activity: UtcTimestamp,
        submitters: Vec<i32>,
        chadsoft_ids: Vec<ChadsoftID>
    ],
    pub Players: [
        bio: Option<String>,
        pronouns: Option<String>,
        region_id: i32,
        joined_date: UtcTimestamp,
        last_activity: UtcTimestamp,
        submitters: (),
        chadsoft_ids: Vec<ChadsoftID>
    ],
    pub PlayersBasic: [
        bio: (),
        pronouns: (),
        region_id: (),
        joined_date: (),
        last_activity: (),
        submitters: (),
        chadsoft_ids: ()
    ],
)]
#[serde_with::skip_serializing_none]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(serde::Deserialize, Debug, Clone, GetId, GetSessionToken, GetCategory)]
#[serde(rename_all = "camelCase")]
pub struct PlayersTemplate {
    #[internal(id)]
    #[wasm_bindgen(readonly)]
    pub id: i32,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub alias: Option<String>,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub bio: either_field::either!(() | Option<String>),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub pronouns: either_field::either!(() | Option<String>),

    #[wasm_bindgen(readonly)]
    pub region_id: either_field::either!(() | i32),

    #[wasm_bindgen(readonly)]
    pub joined_date: either_field::either!(() | UtcTimestamp),

    #[wasm_bindgen(readonly)]
    pub last_activity: either_field::either!(() | UtcTimestamp),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub submitters: either_field::either!(() | Vec<i32>),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub chadsoft_ids: either_field::either!(() | Vec<ChadsoftID>),
}
