use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        category::Category,
        lap_mode::LapMode,
        limit::Limit,
        rankings::{
            AverageFinish, AverageRankRating, PersonalRecordWorldRecord, TallyPoints, TotalTime,
        },
        scores::{Scores, ScoresWithPlayer, Times},
        utc_timestamp::UtcTimestamp,
    },
    endpoint::{RequiredPermission, Root, Scope, scores::ScoresScope},
    request_method::RequestMethod,
};

pub struct TimesheetScope;

impl Scope for TimesheetScope {
    const PATH: &'static str = "/timesheet";

    type OuterScope = ScoresScope;
}

#[derive(Default, Endpoint)]
#[internal(path = "/get", input = GetTimesheetInput, output = Timesheet, scope = TimesheetScope)]
pub struct GetTimesheet;

#[derive(Default, serde::Serialize)]
pub struct Timesheet {
    pub times: Vec<Times>,
    pub af: AverageFinish,
    pub total_time: TotalTime,
    pub tally: TallyPoints,
    pub arr: AverageRankRating,
    pub prwr: PersonalRecordWorldRecord,
}

#[derive(Default, serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct GetTimesheetInput {
    #[internal(id)]
    pub player_id: i32,
    #[internal(category)]
    pub category: Category,
    pub lap_mode: LapMode,
    pub max_date: UtcTimestamp,
    pub region_id: i32,
}

#[derive(Default, Endpoint)]
#[internal(path = "/linechart", input = GetLinechartInput, output = Vec<Scores>, scope = TimesheetScope)]
pub struct GetLinechart;

#[derive(Default, serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct GetLinechartInput {
    #[internal(id)]
    pub player_id: i32,
    #[internal(category)]
    pub category: Category,
    pub track_id: i32,
    pub is_lap: bool,
}

#[derive(Default, Endpoint)]
#[internal(path = "/matchup", input = GetMatchupInput, output = GetMatchupOutput, scope = TimesheetScope)]
pub struct GetMatchup;

#[derive(Default, serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct GetMatchupInput {
    pub player_ids: Vec<i32>,
    #[internal(category)]
    pub category: Category,
    pub lap_mode: LapMode,
    pub is_lap: bool,
    pub max_date: UtcTimestamp,
    pub region_id: i32,
}

// TODO: Make all these vectors related and not need to index them.
#[derive(serde::Serialize, Default)]
pub struct GetMatchupOutput {
    pub player_data: Vec<Timesheet>,
    pub wins: Vec<i8>,
    pub diff_first: Vec<Vec<i32>>,
    pub diff_next: Vec<Vec<i32>>,
    pub diff_af_first: Vec<f64>,
    pub diff_af_next: Vec<f64>,
    pub diff_total_time_first: Vec<i32>,
    pub diff_total_time_next: Vec<i32>,
    pub diff_tally_first: Vec<i16>,
    pub diff_tally_next: Vec<i16>,
    pub diff_arr_first: Vec<f64>,
    pub diff_arr_next: Vec<f64>,
    pub diff_prwr_first: Vec<f64>,
    pub diff_prwr_next: Vec<f64>,
    pub diff_wins_first: Vec<i8>,
    pub diff_wins_next: Vec<i8>,
    pub rgb_diff: Vec<Vec<u8>>,
    pub rgb_diff_af: Vec<u8>,
    pub rgb_diff_total_time: Vec<u8>,
    pub rgb_diff_tally: Vec<u8>,
    pub rgb_diff_arr: Vec<u8>,
    pub rgb_diff_prwr: Vec<u8>,
    pub rgb_diff_wins: Vec<u8>,
}
