use crate::request_method;

pub mod cups;

pub enum RequiredPermission {
    None,
    LoggedIn,
    Admin,
}

pub trait Endpoint {
    const PATH: &'static str;
    const REQUEST_METHOD: request_method::RequestMethod;
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

/// This should be used as the "root" scope.
impl Scope for () {
    const PATH: &'static str = "";
    type OuterScope = ();
}
