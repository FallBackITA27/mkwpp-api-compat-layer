use mkwpp_api_compat_layer_core::{
    common_types::NoData,
    compatibility_layer::typescript_wasm::Fetchable,
    endpoint::{
        champs::{GetChamps, GetChampsFilters},
        cups::GetCups,
        regions::GetRegionsWithPlayerCount,
        standard_levels::GetStandardLevels,
        standards::GetStandards,
        tracks::GetTracks,
        Endpoint,
    },
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ApiHandler;

#[wasm_bindgen]
impl ApiHandler {
    /// Raw Cups from DB
    pub async fn fetch_get_cups() -> Result<<GetCups as Endpoint>::OutputStruct, JsValue> {
        GetCups::fetch(NoData).await
    }

    /// Raw Tracks from DB
    pub async fn fetch_get_tracks() -> Result<<GetTracks as Endpoint>::OutputStruct, JsValue> {
        GetTracks::fetch(NoData).await
    }

    /// Raw Standards from DB
    pub async fn fetch_get_standards() -> Result<<GetStandards as Endpoint>::OutputStruct, JsValue>
    {
        GetStandards::fetch(NoData).await
    }

    /// Raw Standard Levels from DB
    pub async fn fetch_get_standard_levels(
    ) -> Result<<GetStandardLevels as Endpoint>::OutputStruct, JsValue> {
        GetStandardLevels::fetch(NoData).await
    }

    /// Site Champs Getter
    pub async fn fetch_get_champs(
        filters: GetChampsFilters,
    ) -> Result<<GetChamps as Endpoint>::OutputStruct, JsValue> {
        GetChamps::fetch(filters).await
    }

    /// Get Regions with (collapsed) Player Counts
    pub async fn fetch_get_regions_with_player_counts(
    ) -> Result<<GetRegionsWithPlayerCount as Endpoint>::OutputStruct, JsValue> {
        GetRegionsWithPlayerCount::fetch(NoData).await
    }
}
