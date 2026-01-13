use web_sys::RequestInit;

use crate::{common_types::NoData, endpoint::champs::GetChampsFilters};

pub struct FetchData {
    pub(super) query_string: String,
    pub(super) body: Option<String>,
}

impl Default for FetchData {
    fn default() -> Self {
        FetchData {
            query_string: String::new(),
            body: None,
        }
    }
}

pub trait InputToRequest {
    fn to_input(self) -> FetchData;
}

impl InputToRequest for NoData {
    fn to_input(self) -> FetchData {
        Default::default()
    }
}

impl InputToRequest for GetChampsFilters {
    fn to_input(self) -> FetchData {
        match self.category {
            None => Default::default(),
            Some(v) => {
                let category: u8 = v.into();
                FetchData {
                    query_string: format!("?cat={category}"),
                    body: None,
                }
            }
        }
    }
}
