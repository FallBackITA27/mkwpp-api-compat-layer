use crate::{
    common_data_traits::DataHasId,
    endpoint::{Endpoint, RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub struct CupsScope;

impl Scope for CupsScope {
    const PATH: &'static str = "/cups";

    type OuterScope = Root;
}

#[derive(Default)]
pub struct GetCups;

impl Endpoint for GetCups {
    const PATH: &'static str = "/get";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = ();
    type OutputStruct = Vec<GetCupsOutput>;

    type ScopeStruct = CupsScope;
}

#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
pub struct GetCupsOutput {
    pub id: i32,
    pub code: &'static str,
    pub track_ids: [i32; 4],
}

impl DataHasId for GetCupsOutput {
    fn get_id(&self) -> i32 {
        self.id
    }
}
