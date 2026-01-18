use crate::{
    common_data_traits::DataTraits, request_method::RequestMethod,
    required_permission::RequiredPermission,
};

pub mod blog;
pub mod champs;
pub mod cups;
pub mod players;
pub mod rankings;
pub mod regions;
pub mod scores;
pub mod standard_levels;
pub mod standards;
pub mod tracks;

pub trait Endpoint: Default {
    const PATH: &'static str;
    const REQUEST_METHOD: RequestMethod;
    const REQUIRED_PERMISSION: RequiredPermission;

    type InputStruct: DataTraits;
    type OutputStruct;

    type ScopeStruct: Scope;

    fn construct_full_path() -> String {
        Self::ScopeStruct::construct_full_path() + Self::PATH
    }
}

pub trait Scope {
    const PATH: &'static str;
    type OuterScope: Scope;

    fn construct_full_path() -> String {
        Self::OuterScope::construct_full_path() + Self::PATH
    }
}

pub struct Root;

impl Scope for Root {
    const PATH: &'static str = "/v1";
    type OuterScope = Root;
}
