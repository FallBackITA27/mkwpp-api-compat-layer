use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};
use serde::ser::SerializeMap;

use crate::{
    common_types::regions::{RegionType, Regions, RegionsWithPlayerCount},
    endpoint::{Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct RegionsScope;
impl Scope for RegionsScope {
    const PATH: &'static str = "/regions";
    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_with_player_count", output = Vec<RegionsWithPlayerCount>, scope = RegionsScope)]
pub struct GetRegionsWithPlayerCount;

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_descendants", input = GetRegionsDescAncInput, output = Vec<i32>, scope = RegionsScope)]
pub struct GetRegionsDescendants;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken, Default)]
pub struct GetRegionsDescAncInput {
    #[internal(id)]
    pub id: i32,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_ancestors", input = GetRegionsDescAncInput, output = Vec<i32>, scope = RegionsScope)]
pub struct GetRegionsAncestors;

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/type_hashmap", output = std::collections::HashMap<RegionType, Vec<i32>>, scope = RegionsScope)]
pub struct GetRegionsTypeHashmap;

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/child_tree", output = RegionChildrenTree, scope = RegionsScope)]
pub struct GetRegionsChildrenTree;

#[derive(GetId, GetSessionToken, GetCategory)]
pub struct RegionChildrenTree {
    #[internal(id)]
    pub id: i32,

    pub children: Option<Vec<RegionChildrenTree>>,
}

impl serde::Serialize for RegionChildrenTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.children {
            None => serializer.serialize_i32(self.id),
            Some(ref v) => {
                let mut z = serializer.serialize_map(Some(1))?;
                z.serialize_entry(&self.id, v)?;
                z.end()
            }
        }
    }
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/regions_admin_insert", input = RegionsAdminInsertInput, scope = RegionsScope, request = RequestMethod::Put, required = RequiredPermission::Admin)]
pub struct RegionsAdminInsert;

#[derive(Default, serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct RegionsAdminInsertInput {
    pub code: String,
    pub region_type: RegionType,
    pub parent_id: Option<i32>,
    pub is_ranked: bool,
    #[internal(session_token)]
    pub session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/regions_admin_edit", input = RegionsAdminEditInput, scope = RegionsScope, request = RequestMethod::Patch, required = RequiredPermission::Admin)]
pub struct RegionsAdminEdit;

#[derive(Default, serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct RegionsAdminEditInput {
    #[internal(id)]
    pub id: i32,
    pub code: String,
    pub region_type: RegionType,
    pub parent_id: Option<i32>,
    pub is_ranked: bool,
    #[internal(session_token)]
    pub session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/regions_admin_delete", input = RegionsAdminDeleteInput, scope = RegionsScope, request = RequestMethod::Delete, required = RequiredPermission::Admin)]
pub struct RegionsAdminDelete;

#[derive(Default, serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct RegionsAdminDeleteInput {
    #[internal(id)]
    pub id: i32,
    #[internal(session_token)]
    pub session_token: String,
}
