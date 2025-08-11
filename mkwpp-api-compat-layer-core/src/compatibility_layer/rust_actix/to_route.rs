use actix_web::{HttpResponse, Route, web};

use super::from_input::InputFromActix;
use crate::{
    endpoint::{Endpoint, cups::GetCups, tracks::GetTracks},
    error::{FinalErrorResponse, PPResult},
};

pub trait ToActixRoute: Endpoint
where
    Self::InputStruct: InputFromActix,
    Self::OutputStruct: serde::Serialize,
{
    /// Only call this once, please.
    fn to_actix_route(
        handler: impl AsyncFn(Self::InputStruct) -> PPResult<Self::OutputStruct> + 'static,
    ) -> Route {
        let handler = std::sync::Arc::new(handler);

        let inner_handler = move || {
            let handler = handler.clone();
            async move {
                let input = Self::InputStruct::get_from_request();
                let data = handler(input).await?;
                Ok::<HttpResponse, FinalErrorResponse>(HttpResponse::Ok().json(data))
            }
        };

        web::method(Self::REQUEST_METHOD.into()).to(inner_handler)
    }
}

impl ToActixRoute for GetCups {}
impl ToActixRoute for GetTracks {}
