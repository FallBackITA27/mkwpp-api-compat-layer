use crate::{
    common_types::{
        Category, LapMode,
        players::PlayersBasic,
        rankings::{
            AverageFinish, AverageRankRating, PersonalRecordWorldRecord, TallyPoints, TotalTime,
        },
    },
    endpoint::{Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

use actix_web::cookie::time::UtcOffset;
use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

pub struct RankingsScope;
impl Scope for RankingsScope {
    const PATH: &'static str = "/rankings";
    type OuterScope = Root;
}

macro_rules! rankings_type {
    ($type: ty) => {
        paste::paste! { rankings_type!($type, "[<get_ $type:snake:lower>]"); }
    };
    ($type: ty, $endpoint: literal) => {
        paste::paste! { rankings_type!($type, $endpoint, [<Get $type>], [<Get $type Output>]); }
    };
    ($type: ty, $endpoint: literal, $struct_name: ident, $output_struct_name: ident) => {
        #[derive(Default, Endpoint)]
        #[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
        #[internal(path = $endpoint, input = RankingsInput, output = Vec<$output_struct_name>, scope = RankingsScope)]
        pub struct $struct_name;

        #[derive(GetId, GetCategory, GetSessionToken)]
        pub struct $output_struct_name {
            pub rank: i32,
            pub value: $type,
            pub player: PlayersBasic,
        }
    };
}

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct RankingsInput {
    #[internal(category)]
    pub category: Category,
    pub lap_mode: LapMode,
    pub date: UtcOffset,
    pub region_id: i32,
}

rankings_type!(AverageFinish);
rankings_type!(TotalTime);
rankings_type!(TallyPoints);
rankings_type!(AverageRankRating);
rankings_type!(PersonalRecordWorldRecord);

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_country", input = RankingsInput, output = Vec<GetCountryRankingsOutput>, scope = RankingsScope)]
pub struct GetCountryRankings;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct GetCountryRankingsOutput {
    pub region_id: AverageFinish,
    pub rank: i32,
    pub value: f64,
}
