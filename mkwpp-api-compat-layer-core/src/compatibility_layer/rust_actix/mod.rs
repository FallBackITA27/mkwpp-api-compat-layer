use actix_web::http;

use crate::request_method::RequestMethod;

pub mod to_route;
pub mod to_scope;

impl Into<http::Method> for RequestMethod {
    fn into(self) -> http::Method {
        match self {
            Self::Get => http::Method::GET,
            Self::Head => http::Method::HEAD,
            Self::Post => http::Method::POST,
            Self::Put => http::Method::PUT,
            Self::Delete => http::Method::DELETE,
            Self::Connect => http::Method::CONNECT,
            Self::Options => http::Method::OPTIONS,
            Self::Trace => http::Method::TRACE,
            Self::Patch => http::Method::PATCH,
        }
    }
}
