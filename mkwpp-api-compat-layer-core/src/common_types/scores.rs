use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};

use crate::common_types::{
    category::Category, players::PlayersBasic, rankings::PersonalRecordWorldRecord,
    utc_timestamp::UtcTimestamp,
};

#[either_field::make_template(
    GenStructs: true,
    DeleteTemplate: true,
    OmitEmptyTupleFields: true;
    pub Scores: [
        player_id: i32,
        admin_note: Option<String>
    ],
    pub ScoresWithPlayer: [
        player: PlayersBasic,
        rank: i32,
        prwr: PersonalRecordWorldRecord,
        std_lvl_code: String
    ],
    pub Times: [
        rank: i32,
        prwr: PersonalRecordWorldRecord,
        std_lvl_code: String
    ],
    pub TimesheetTimesetData: [
        player_id: i32
    ],
    pub ScoresByDate: [
        player: PlayersBasic,
        video_link: (),
        ghost_link: (),
        comment: (),
        was_wr: ()
    ],
    pub RankingsTimesetData: [
        id: (),
        category: (),
        player_id: i32,
        date: (),
        was_wr: (),
        video_link: (),
        ghost_link: (),
        comment: ()
    ],
    pub CountryRankingsTimesetData: [
        id: (),
        category: (),
        player_id: i32,
        region_id: i32,
        date: (),
        was_wr: (),
        video_link: (),
        ghost_link: (),
        comment: ()
    ]
)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
#[derive(Clone, GetId, GetCategory, GetSessionToken)]
#[serde(rename_all = "camelCase")]
pub struct ScoresTemplate {
    #[wasm_bindgen(readonly)]
    #[internal(id)]
    pub id: either_field::either!(i32 | ()),

    #[wasm_bindgen(readonly)]
    pub value: either_field::either!(i32 | ()),

    #[wasm_bindgen(readonly)]
    pub rank: either_field::either!(() | i32),

    #[wasm_bindgen(readonly)]
    pub prwr: either_field::either!(() | PersonalRecordWorldRecord),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub std_lvl_code: either_field::either!(() | String),

    #[wasm_bindgen(readonly)]
    #[internal(Category)]
    pub category: either_field::either!(Category | ()),

    #[wasm_bindgen(readonly)]
    pub is_lap: either_field::either!(bool | ()),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub player: either_field::either!(() | PlayersBasic),

    #[wasm_bindgen(readonly)]
    pub player_id: either_field::either!(() | i32),

    #[wasm_bindgen(readonly)]
    pub region_id: either_field::either!(() | i32),

    #[wasm_bindgen(readonly)]
    pub track_id: either_field::either!(i32 | ()),

    #[wasm_bindgen(readonly)]
    pub date: either_field::either!(Option<UtcTimestamp> | ()),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub video_link: either_field::either!(Option<String> | ()),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub ghost_link: either_field::either!(Option<String> | ()),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub comment: either_field::either!(Option<String> | ()),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub admin_note: either_field::either!(() | Option<String>),

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub was_wr: either_field::either!(bool | ()),
}
