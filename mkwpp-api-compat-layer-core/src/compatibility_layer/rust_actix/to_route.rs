use actix_web::{FromRequest, Handler, Responder, Route, web};

use crate::endpoint::{Endpoint, cups::GetCups};

pub trait ToActixRoute: Endpoint {
    fn to_actix_route<F, Args>(handler: F) -> Route
    where
        F: Handler<Args>,
        Args: FromRequest + 'static,
        F::Output: Responder + 'static,
    {
        web::method(Self::REQUEST_METHOD.into()).to(handler)
    }
}

impl ToActixRoute for GetCups {}
