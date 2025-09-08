use mkwpp_api_compat_layer_core::{
    common_types::NoData,
    compatibility_layer::typescript_wasm::Fetchable,
    endpoint::{cups::GetCups, tracks::GetTracks, Endpoint},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ApiHandler;

#[wasm_bindgen]
impl ApiHandler {
    pub async fn fetch_get_cups() -> Result<<GetCups as Endpoint>::OutputStruct, JsValue> {
        GetCups::fetch(NoData).await
    }

    pub async fn fetch_get_tracks() -> Result<<GetTracks as Endpoint>::OutputStruct, JsValue> {
        GetTracks::fetch(NoData).await
    }
}
