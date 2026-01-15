use crate::{
    common_types::{
        NoData,
        players::PlayersBasic,
        rankings::{
            AverageFinish, AverageRankRating, PersonalRecordWorldRecord, TallyPoints, TotalTime,
        },
    },
    endpoint::{Endpoint, Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

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
        #[derive(Default)]
        #[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
        pub struct $struct_name;

        impl Endpoint for $struct_name {
            const PATH: &'static str = $endpoint;
            const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
            const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

            type InputStruct = NoData;
            type OutputStruct = Vec<$output_struct_name>;

            type ScopeStruct = RankingsScope;
        }

        pub struct $output_struct_name {
            pub rank: i32,
            pub value: $type,
            pub player: PlayersBasic,
        }
    };
}

rankings_type!(AverageFinish);
rankings_type!(TotalTime);
rankings_type!(TallyPoints);
rankings_type!(AverageRankRating);
rankings_type!(PersonalRecordWorldRecord);
