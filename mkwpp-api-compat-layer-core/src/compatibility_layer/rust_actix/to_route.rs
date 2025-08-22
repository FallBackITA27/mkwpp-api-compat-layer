use actix_web::{HttpRequest, HttpResponse, Route, web};

use super::from_input::InputFromActix;
use crate::{
    endpoint::{Endpoint, cups::GetCups, tracks::GetTracks},
    error::{FinalErrorResponse, PPResult},
};

pub struct BasicInputs<T: InputFromActix> {
    inner: T,
    session_token_bytes: Option<[u8; 128]>,
}

impl<T: InputFromActix> InputFromActix for BasicInputs<T> {
    fn get_from_request(request: HttpRequest) -> Self {
        let token = request
            .headers()
            .get("Bearer-Token")
            .map(|value| value.as_bytes().try_into().ok())
            .flatten();
        
        Self {
            inner: T::get_from_request(request),
            session_token_bytes: token,
        }
    }
}

pub trait ToActixRoute: Endpoint
where
    Self::InputStruct: InputFromActix,
    Self::OutputStruct: serde::Serialize,
{
    /// Only call this once, please.
    fn to_actix_route(
        handler: impl AsyncFn(BasicInputs<Self::InputStruct>) -> PPResult<Self::OutputStruct> + 'static,
    ) -> Route {
        let handler = std::sync::Arc::new(handler);

        let inner_handler = move |req: HttpRequest| {
            let handler = handler.clone();
            async move {
                let input = BasicInputs::get_from_request(req);
                let data = handler(input).await?;
                Ok::<HttpResponse, FinalErrorResponse>(HttpResponse::Ok().json(data))
            }
        };

        web::method(Self::REQUEST_METHOD.into()).to(inner_handler)
    }
}

impl ToActixRoute for GetCups {}
impl ToActixRoute for GetTracks {}
