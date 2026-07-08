use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        ChadsoftID,
        players::{Players, PlayersBasic, PlayersLoggedIn},
        user::UserIdentificationData,
        utc_timestamp::UtcTimestamp,
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

#[derive(serde::Deserialize, GetId, GetSessionToken, GetCategory, Default)]
pub struct GetPlayersInput {
    pub basic: bool,
    pub ids: Vec<i32>,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum GetPlayersOutput {
    #[default]
    None,
    NormalOne(Players),
    NormalVec(Vec<Players>),
    BasicOne(PlayersBasic),
    BasicVec(Vec<PlayersBasic>),
    // LoggedInOne(PlayersLoggedIn),
    // LoggedInVec(Vec<PlayersLoggedIn>),
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/list", output = Vec<PlayersBasic>, scope = PlayersScope)]
pub struct GetPlayersList;

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/update_bio", input = UpdateBioInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct UpdateBio;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct UpdateBioInput {
    pub data: String,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/update_alias", input = UpdateAliasInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct UpdateAlias;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct UpdateAliasInput {
    pub data: String,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/update_pronouns", input = UpdatePronounsInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct UpdatePronouns;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct UpdatePronounsInput {
    pub data: String,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_submitters", input = UserIdentificationData, output = GetSubmittersOutput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct GetSubmitters;

#[derive(Default, serde::Serialize, GetId, GetCategory, GetSessionToken)]
pub struct GetSubmittersOutput {
    pub player_ids: Vec<i32>,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_submittees", input = UserIdentificationData, output = GetSubmitteesOutput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct GetSubmittees;

#[derive(Default, serde::Serialize, GetId, GetCategory, GetSessionToken)]
pub struct GetSubmitteesOutput {
    pub player_ids: Vec<i32>,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/add_submitter", input = AddSubmitterInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct AddSubmitter;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AddSubmitterInput {
    pub player_id: i32,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/remove_submitter", input = RemoveSubmitterInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct RemoveSubmitter;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct RemoveSubmitterInput {
    pub player_id: i32,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/set_submitters", input = SetSubmittersInput, scope = PlayersScope, required = RequiredPermission::LoggedIn)]
pub struct SetSubmitters;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct SetSubmittersInput {
    pub player_ids: Vec<i32>,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_admin_player_list", input = GetAdminPlayerListInput, output = Vec<PlayersLoggedIn>, scope = PlayersScope, required = RequiredPermission::Admin, request = RequestMethod::Post)]
pub struct GetAdminPlayerList;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct GetAdminPlayerListInput {
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_player_insert", input = AdminPlayerInsertInput, scope = PlayersScope, required = RequiredPermission::Admin, request = RequestMethod::Put)]
pub struct AdminPlayerInsert;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminPlayerInsertInput {
    pub name: String,
    pub alias: Option<String>,
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub region_id: i32,
    pub joined_date: UtcTimestamp,
    pub last_activity: UtcTimestamp,
    pub submitters: Vec<i32>,
    pub chadsoft_ids: Vec<ChadsoftID>,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_player_edit", input = AdminPlayerEditInput, scope = PlayersScope, required = RequiredPermission::Admin, request = RequestMethod::Patch)]
pub struct AdminPlayerEdit;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminPlayerEditInput {
    #[internal(id)]
    pub id: i32,
    pub name: String,
    pub alias: Option<String>,
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub region_id: i32,
    pub joined_date: UtcTimestamp,
    pub last_activity: UtcTimestamp,
    pub submitters: Vec<i32>,
    pub chadsoft_ids: Vec<ChadsoftID>,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_player_delete", input = AdminPlayerDeleteInput, scope = PlayersScope, required = RequiredPermission::Admin, request = RequestMethod::Delete)]
pub struct AdminPlayerDelete;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminPlayerDeleteInput {
    #[internal(id)]
    id: i32,
    #[internal(session_token)]
    session_token: String,
}
