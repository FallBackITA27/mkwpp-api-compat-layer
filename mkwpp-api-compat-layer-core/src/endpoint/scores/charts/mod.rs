use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken, InputFromActix};

use crate::{
    common_types::{UtcTimestamp, category::Category, limit::Limit, scores::ScoresWithPlayer},
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

#[derive(GetId, GetSessionToken, GetCategory, InputFromActix)]
pub struct GetChartsInput {
    #[internal(id, required)]
    pub id: i32,

    #[internal(category, key = "cat")]
    pub category: Category,

    #[internal(key = "lap")]
    pub is_lap: bool,

    #[internal(key = "dat")]
    pub max_date: UtcTimestamp,

    #[internal(key = "reg")]
    pub region_id: i32,

    #[internal(key = "lim")]
    pub limit: Limit,
}

#[derive(Default, Endpoint)]
#[internal(path = "/get_dates", input = GetChartsDatesInput, output = Vec<UtcTimestamp>, scope = ChartsScope)]
pub struct GetChartsDates;

#[derive(GetId, GetSessionToken, GetCategory)]
pub struct GetChartsDatesInput {
    #[internal(id)]
    id: i32,
    #[internal(category)]
    category: Category,
    is_lap: bool,
    region_id: i32,
}
