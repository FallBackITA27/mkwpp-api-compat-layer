use std::str::FromStr;

use crate::{
    common_types::{NoData, UtcTimestamp, category::Category, limit::Limit},
    endpoint::{regions::GetRegionsDescAncInput, scores::charts::GetChartsInput},
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

        if limit.is_some() {
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
                None => {
                    return Err(ErrorCodes::InvalidInput
                        .into_final_error(None::<&str>, file!(), line!())
                        .into_response(StatusCode::BadRequest));
                }
            },
        })
    }
}

impl InputFromActix for GetChartsInput {
    // id: i32,
    // category: Category,
    // is_lap: bool,
    // max_date: UtcTimestamp,
    // region_id: i32,
    // limit: Limit,
    fn get_from_request(request: &mut HttpRequest) -> Result<Self, FinalErrorResponse> {
        let id = request.query_string().split(&['?', '&']).fold(
            (None, None, None, None, None, None),
            |mut acc, next| {
                let mut split = next.split('=');
                match split.next() {
                    Some("id") => acc.0 = split.next().map(FromStr::<i32>::from_str).flatten(),
                    Some("cat") => acc.1 = split.next(),
                    Some("lap") => acc.2 = split.next(),
                    Some("dat") => acc.3 = split.next(),
                    Some("reg") => acc.4 = split.next(),
                    Some("lim") => acc.5 = split.next(),
                    _ => (),
                };
                acc
            },
        );

        Ok(Self {
            id: 9,
            category: Category::Normal,
            is_lap: false,
            max_date: UtcTimestamp::from(10),
            region_id: 0,
            limit: Limit::from(10),
        })
    }
}
