use crate::{
    common_types::{NoData, category::Category, limit::Limit},
    endpoint::{regions::GetRegionsDescAncInput},
    error::{ErrorCodes, FinalErrorResponse},
    status_code::StatusCode,
};
use actix_web::HttpRequest;

pub trait InputFromActix: Sized {
    fn get_from_request(request: &mut HttpRequest) -> Result<Self, FinalErrorResponse>;
}

impl InputFromActix for NoData {
    fn get_from_request(_: &mut HttpRequest) -> Result<Self, FinalErrorResponse> {
        Ok(NoData)
    }
}

impl InputFromActix for Option<Category> {
    fn get_from_request(request: &mut HttpRequest) -> Result<Self, FinalErrorResponse> {
        let category = request
            .query_string()
            .split(&['?', '&'])
            .filter_map(|v| {
                let mut split = v.split('=');
                match split.next() {
                    Some("cat") => split.next(),
                    _ => None,
                }
            })
            .filter_map(|v| v.parse::<u8>().ok())
            .filter_map(|v| <u8 as TryInto<Category>>::try_into(v).ok())
            .next_back();

        if category.is_some() {
            return Ok(category);
        }

        /* Insert here checks for body */

        Ok(None)
    }
}

impl InputFromActix for Category {
    fn get_from_request(request: &mut HttpRequest) -> Result<Self, FinalErrorResponse> {
        <Option<Category>>::get_from_request(request).map(|v| v.unwrap_or_default())
    }
}

impl InputFromActix for Option<Limit> {
    fn get_from_request(request: &mut HttpRequest) -> Result<Self, FinalErrorResponse> {
        let limit = request
            .query_string()
            .split(&['?', '&'])
            .filter_map(|v| {
                let mut split = v.split('=');
                match split.next() {
                    Some("lim") => split.next(),
                    _ => None,
                }
            })
            .filter_map(|v| v.parse::<i32>().ok())
            .map(From::from)
            .next_back();

        if category.is_some() {
            return Ok(limit);
        }

        /* Insert here checks for body */

        Ok(None)
    }
}

impl InputFromActix for Limit {
    fn get_from_request(request: &mut HttpRequest) -> Result<Self, FinalErrorResponse> {
        <Option<Limit>>::get_from_request(request).map(|v| v.unwrap_or_default())
    }
}

impl InputFromActix for GetRegionsDescAncInput {
    fn get_from_request(request: &mut HttpRequest) -> Result<Self, FinalErrorResponse> {
        let id = request
            .query_string()
            .split(&['?', '&'])
            .filter_map(|v| {
                let mut split = v.split('=');
                match split.next() {
                    Some("id") => split.next(),
                    _ => None,
                }
            })
            .filter_map(|v| v.parse::<i32>().ok())
            .next_back();

        Ok(Self {
            id: match id {
                Some(v) => v,
                None => return Err(ErrorCodes::InvalidInput
                    .into_final_error(None::<&str>, file!(), line!())
                    .into_response(StatusCode::BadRequest)),
            },
        })
    }
}
