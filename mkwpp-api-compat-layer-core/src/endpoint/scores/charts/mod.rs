use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        category::Category, limit::Limit, scores::ScoresWithPlayer, utc_timestamp::UtcTimestamp,
    },
    endpoint::{RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub struct ChartsScope;

impl Scope for ChartsScope {
    const PATH: &'static str = "/charts";

    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[internal(path = "/get", input = GetChartsInput, output = Vec<ScoresWithPlayer>, scope = ChartsScope)]
pub struct GetCharts;

#[derive(Default, serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct GetChartsInput {
    #[internal(id)]
    pub track_id: i32,
    #[internal(category)]
    pub category: Category,
    pub is_lap: bool,
    pub max_date: UtcTimestamp,
    pub region_id: i32,
    pub limit: Limit,
}

#[derive(Default, Endpoint)]
#[internal(path = "/get_dates", input = GetChartsDatesInput, output = Vec<UtcTimestamp>, scope = ChartsScope)]
pub struct GetChartsDates;

#[derive(serde::Deserialize, Default, GetId, GetSessionToken, GetCategory)]
pub struct GetChartsDatesInput {
    #[internal(id)]
    pub track_id: i32,
    #[internal(category)]
    pub category: Category,
    pub is_lap: bool,
    pub region_id: i32,
}
