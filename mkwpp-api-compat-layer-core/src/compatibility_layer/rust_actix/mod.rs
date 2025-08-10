use actix_web::http;

use crate::request_method::RequestMethod;

pub mod error;
pub mod from_input;
pub mod to_route;
pub mod to_scope;
pub mod common_types;

impl From<RequestMethod> for http::Method {
    fn from(val: RequestMethod) -> Self {
        match val {
            RequestMethod::Get => http::Method::GET,
            RequestMethod::Head => http::Method::HEAD,
            RequestMethod::Post => http::Method::POST,
            RequestMethod::Put => http::Method::PUT,
            RequestMethod::Delete => http::Method::DELETE,
            RequestMethod::Connect => http::Method::CONNECT,
            RequestMethod::Options => http::Method::OPTIONS,
            RequestMethod::Trace => http::Method::TRACE,
            RequestMethod::Patch => http::Method::PATCH,
        }
    }
}
