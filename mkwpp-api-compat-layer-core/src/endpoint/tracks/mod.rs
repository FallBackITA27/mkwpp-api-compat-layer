use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::Category,
    endpoint::{RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub struct TracksScope;

impl Scope for TracksScope {
    const PATH: &'static str = "/tracks";

    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[internal(path = "/get", output = Vec<GetTracksOutput>, scope = TracksScope)]
pub struct GetTracks;

#[derive(GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct GetTracksOutput {
    #[wasm_bindgen(readonly)]
    #[internal(id)]
    pub id: i32,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub abbr: String,

    #[wasm_bindgen(readonly)]
    pub cup_id: i32,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub categories: Vec<Category>,
}
