use crate::{common_types::NoData, endpoint::champs::GetChampsFilters};

#[derive(Default)]
pub struct FetchData {
    pub(super) query_string: String,
    pub(super) body: Option<String>,
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
