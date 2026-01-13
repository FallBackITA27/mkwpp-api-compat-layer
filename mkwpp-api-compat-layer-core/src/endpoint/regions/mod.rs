use serde::ser::SerializeMap;

use crate::{
    common_data_traits::{GetId, GetSessionToken, HasId},
    common_types::{
        Category, NoData, UtcTimestamp,
        regions::{RegionType, Regions, RegionsWithPlayerCount},
    },
    endpoint::{Endpoint, Root, Scope},
    request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub struct RegionsScope;
impl Scope for RegionsScope {
    const PATH: &'static str = "/regions";
    type OuterScope = Root;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetRegionsWithPlayerCount;

impl Endpoint for GetRegionsWithPlayerCount {
    const PATH: &'static str = "/get_with_player_count";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = Vec<RegionsWithPlayerCount>;

    type ScopeStruct = RegionsScope;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetRegionsDescendants;

impl Endpoint for GetRegionsDescendants {
    const PATH: &'static str = "/get_descendants";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = Vec<i32>;

    type ScopeStruct = RegionsScope;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetRegionsAncestors;

impl Endpoint for GetRegionsAncestors {
    const PATH: &'static str = "/get_ancestors";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = Vec<i32>;

    type ScopeStruct = RegionsScope;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetRegionsTypeHashmap;

impl Endpoint for GetRegionsTypeHashmap {
    const PATH: &'static str = "/type_hashmap";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = std::collections::HashMap<RegionType, Vec<Regions>>;

    type ScopeStruct = RegionsScope;
}

#[derive(Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct GetRegionsChildrenTree;

impl Endpoint for GetRegionsChildrenTree {
    const PATH: &'static str = "/child_tree";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = NoData;
    type OutputStruct = RegionChildrenTree;

    type ScopeStruct = RegionsScope;
}

pub struct RegionChildrenTree {
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
