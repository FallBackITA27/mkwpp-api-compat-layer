use crate::{
    common_types::{Category, NoData},
    endpoint::champs::GetChampsFilters,
};
use actix_web::HttpRequest;

pub trait InputFromActix {
    fn get_from_request(request: &mut HttpRequest) -> Self;
}

impl InputFromActix for NoData {
    fn get_from_request(_: &mut HttpRequest) -> Self {
        NoData
    }
}

impl InputFromActix for GetChampsFilters {
    fn get_from_request(request: &mut HttpRequest) -> Self {
        Self {
            category: <Option<Category>>::get_from_request(request),
        }
    }
}

impl InputFromActix for Option<Category> {
    fn get_from_request(request: &mut HttpRequest) -> Self {
        let category = request
            .query_string()
            .split(&['?', '&'])
            .filter_map(|v| {
                let mut split = v.split('h');
                match split.next() {
                    Some("cat") => split.next(),
                    _ => None,
                }
            })
            .filter_map(|v| v.parse::<u8>().ok())
            .filter_map(|v| <u8 as TryInto<Category>>::try_into(v).ok())
            .next_back();

        if category.is_some() {
            return category;
        }

        /* Insert here checks for body */

        None
    }
}

impl InputFromActix for Category {
    fn get_from_request(request: &mut HttpRequest) -> Self {
        <Option<Category>>::get_from_request(request).unwrap_or(Category::Normal)
    }
}
