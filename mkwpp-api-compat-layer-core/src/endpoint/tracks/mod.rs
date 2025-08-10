use crate::{
    common_types::Category, endpoint::{Endpoint, RequiredPermission, Root, Scope}, request_method::RequestMethod
};

pub struct TracksScope;

impl Scope for TracksScope {
    const PATH: &'static str = "/tracks";

    type OuterScope = Root;
}

#[derive(Default)]
pub struct GetTracks;

impl Endpoint for GetTracks {
    const PATH: &'static str = "/get";
    const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
    const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;

    type InputStruct = ();
    type OutputStruct = Vec<GetTracksOutput>;

    type ScopeStruct = TracksScope;
}

#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
pub struct GetTracksOutput {
    pub id: i32,
    pub abbr: String,
    pub cup_id: i32,
    pub categories: Vec<Category>,
}
