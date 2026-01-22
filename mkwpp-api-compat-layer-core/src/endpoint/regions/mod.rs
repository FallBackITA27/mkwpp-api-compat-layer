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

#[derive(GetId, GetCategory, GetSessionToken)]
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
#[internal(path = "/type_hashmap", output = std::collections::HashMap<RegionType, Vec<Regions>>, scope = RegionsScope)]
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
