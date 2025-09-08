use crate::common_types::NoData;
use actix_web::HttpRequest;

pub trait InputFromActix {
    fn get_from_request(request: HttpRequest) -> Self;
}

impl InputFromActix for NoData {
    fn get_from_request(_: HttpRequest) -> Self {
        NoData
    }
}
