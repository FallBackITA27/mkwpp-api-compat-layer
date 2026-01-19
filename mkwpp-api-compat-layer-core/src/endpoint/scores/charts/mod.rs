use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{category::Category, UtcTimestamp, limit::Limit, scores::ScoresWithPlayer},
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

#[derive(GetId, GetSessionToken, GetCategory)]
pub struct GetChartsInput {
    #[internal(id)]
    id: i32,
    #[internal(category)]
    category: Category,
    is_lap: bool,
    max_date: UtcTimestamp,
    region_id: i32,
    limit: Limit,
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
