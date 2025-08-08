use crate::{request_method::RequestMethod, required_permission::RequiredPermission};

pub mod cups;
pub mod players;

pub trait Endpoint {
    const PATH: &'static str;
    const REQUEST_METHOD: RequestMethod;
    const REQUIRED_PERMISSION: RequiredPermission;

    type InputStruct;
    type OutputStruct;

    type ScopeStruct: Scope;
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
    const PATH: &'static str = "";
    type OuterScope = Root;
}
