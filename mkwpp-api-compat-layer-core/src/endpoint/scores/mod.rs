use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        category::Category,
        lap_mode::LapMode,
        limit::Limit,
        scores::{Scores, ScoresByDate, ScoresWithPlayer},
        utc_timestamp::UtcTimestamp,
    },
    endpoint::{RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub mod charts;
pub mod timesheet;

pub struct ScoresScope;

impl Scope for ScoresScope {
    const PATH: &'static str = "/scores";

    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[internal(path = "/recent", input = GetRecentScoresInput, output = Vec<ScoresByDate>, scope = ScoresScope)]
pub struct GetRecentScores;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken, Default)]
pub struct GetRecentScoresInput {
    pub world_records: bool,
    pub limit: Limit,
}

#[derive(Default, Endpoint)]
#[internal(path = "/records", input = GetRecordsInput, output = Vec<ScoresWithPlayer>, scope = ScoresScope)]
pub struct GetRecords;

#[derive(serde::Deserialize, Default, GetId, GetCategory, GetSessionToken)]
pub struct GetRecordsInput {
    #[internal(id)]
    pub track_id: i32,

    #[internal(category)]
    pub category: Category,

    pub lap_mode: LapMode,

    pub max_date: UtcTimestamp,

    pub region_id: i32,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_admin_score_list", input = GetAdminScoreListInput, output = Vec<Scores>, scope = ScoresScope, required = RequiredPermission::Admin, request = RequestMethod::Post)]
pub struct GetAdminScoreList;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct GetAdminScoreListInput {
    pub track_id: i32,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_admin_score", input = GetAdminScoreInput, output = Scores, scope = ScoresScope, required = RequiredPermission::Admin, request = RequestMethod::Post)]
pub struct GetAdminScore;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct GetAdminScoreInput {
    #[internal(id)]
    id: i32,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_score_insert", input = AdminScoreInsertInput, scope = ScoresScope, required = RequiredPermission::Admin, request = RequestMethod::Put)]
pub struct AdminScoreInsert;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminScoreInsertInput {
    pub value: i32,
    pub category: Category,
    pub is_lap: bool,
    pub player_id: i32,
    pub track_id: i32,
    pub date: Option<UtcTimestamp>,
    pub video_link: Option<String>,
    pub ghost_link: Option<String>,
    pub comment: Option<String>,
    pub admin_note: Option<String>,
    #[internal(session_token)]
    pub session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_score_edit", input = AdminScoreEditInput, scope = ScoresScope, required = RequiredPermission::Admin, request = RequestMethod::Patch)]
pub struct AdminScoreEdit;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminScoreEditInput {
    pub value: i32,
    pub category: Category,
    pub is_lap: bool,
    pub player_id: i32,
    pub track_id: i32,
    pub date: Option<UtcTimestamp>,
    pub video_link: Option<String>,
    pub ghost_link: Option<String>,
    pub comment: Option<String>,
    pub admin_note: Option<String>,
    #[internal(session_token)]
    pub session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_score_delete", input = AdminScoreDeleteInput, scope = ScoresScope, required = RequiredPermission::Admin, request = RequestMethod::Delete)]
pub struct AdminScoreDelete;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminScoreDeleteInput {
    #[internal(id)]
    id: i32,
    #[internal(session_token)]
    session_token: String,
}
