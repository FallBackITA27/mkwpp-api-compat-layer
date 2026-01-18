use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        NoData,
        players::{Players, PlayersBasic},
    },
    endpoint::{Endpoint, Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct PlayersScope;
impl Scope for PlayersScope {
    const PATH: &'static str = "/players";

    type OuterScope = Root;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetPlayers;

impl Endpoint for GetPlayers {
    const PATH: &'static str = "/get";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = GetPlayersInput;
    type OutputStruct = GetPlayersOutput;

    type ScopeStruct = PlayersScope;
}

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

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetList;

impl Endpoint for GetList {
    const PATH: &'static str = "/list";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = Vec<PlayersBasic>;

    type ScopeStruct = PlayersScope;
}
