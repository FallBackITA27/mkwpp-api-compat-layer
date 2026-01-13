use crate::{
    common_data_traits::{GetCategory, GetId, GetSessionToken},
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

struct GetPlayersInput {
    basic: bool,
}
impl GetId for GetPlayersInput {}
impl GetCategory for GetPlayersInput {}
impl GetSessionToken for GetPlayersInput {}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum GetPlayersOutput {
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
