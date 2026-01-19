use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        category::Category, lap_mode::LapMode, UtcTimestamp,
        limit::Limit,
        scores::{ScoresByDate, ScoresWithPlayer},
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

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct GetRecentScoresInput {
    pub world_records: bool,
    pub limit: Limit,
}

#[derive(Default, Endpoint)]
#[internal(path = "/records", input = GetRecordsInput, output = Vec<ScoresWithPlayer>, scope = ScoresScope)]
pub struct GetRecords;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct GetRecordsInput {
    #[internal(id)]
    pub track_id: i32,

    #[internal(category)]
    pub category: Category,

    pub lap_mode: LapMode,

    pub max_date: UtcTimestamp,

    pub region_id: i32,
}
