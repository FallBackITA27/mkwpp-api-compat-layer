use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::players::{Players, PlayersBasic},
    endpoint::{Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct PlayersScope;
impl Scope for PlayersScope {
    const PATH: &'static str = "/players";

    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get", input = GetPlayersInput, output = GetPlayersOutput, scope = PlayersScope)]
pub struct GetPlayers;

#[derive(GetId, GetSessionToken, GetCategory)]
pub struct GetPlayersInput {
    basic: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum GetPlayersOutput {
    NormalOne(Players),
    NormalVec(Vec<Players>),
    BasicOne(PlayersBasic),
    BasicVec(Vec<PlayersBasic>),
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/list", output = Vec<PlayersBasic>, scope = PlayersScope)]
pub struct GetList;
