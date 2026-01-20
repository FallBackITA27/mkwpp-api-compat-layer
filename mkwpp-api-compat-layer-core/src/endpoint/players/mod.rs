use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        players::{Players, PlayersBasic, PlayersLoggedIn},
        user::UserIdentificationData,
    },
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
    ids: Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum GetPlayersOutput {
    NormalOne(Players),
    NormalVec(Vec<Players>),
    BasicOne(PlayersBasic),
    BasicVec(Vec<PlayersBasic>),
    LoggedInOne(PlayersLoggedIn),
    LoggedInVec(Vec<PlayersLoggedIn>),
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/list", output = Vec<PlayersBasic>, scope = PlayersScope)]
pub struct GetList;

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/update_bio", input = UpdateBioInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct UpdateBio;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct UpdateBioInput {
    data: String,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/update_alias", input = UpdateAliasInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct UpdateAlias;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct UpdateAliasInput {
    data: String,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/update_pronouns", input = UpdatePronounsInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct UpdatePronouns;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct UpdatePronounsInput {
    data: String,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_submitters", input = UserIdentificationData, output = GetSubmittersOutput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct GetSubmitters;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct GetSubmittersOutput {
    player_ids: Vec<i32>,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_submittees", input = UserIdentificationData, output = GetSubmitteesOutput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct GetSubmittees;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct GetSubmitteesOutput {
    player_ids: Vec<i32>,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/add_submitter", input = AddSubmitterInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct AddSubmitter;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct AddSubmitterInput {
    player_id: i32,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/remove_submitter", input = RemoveSubmitterInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct RemoveSubmitter;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct RemoveSubmitterInput {
    player_id: i32,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/set_submitters", input = SetSubmittersInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct SetSubmitters;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct SetSubmittersInput {
    player_ids: Vec<i32>,
    #[internal(session_token)]
    session_token: String,
}
