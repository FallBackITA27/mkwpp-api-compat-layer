use crate::status_code::StatusCode;

#[cfg_attr(feature = "rust-actix", derive(Debug, serde::Serialize))]
pub struct FinalErrorResponse {
    #[cfg_attr(feature = "rust-actix", serde(skip))]
    pub status_code: StatusCode,
    pub error_code: u64,
    pub non_field_errors: Vec<String>,
    pub field_errors: std::collections::HashMap<String, Vec<String>>,
}

impl FinalErrorResponse {
    pub fn new(
        error_code: u64,
        status_code: StatusCode,
        non_field_errors: Vec<String>,
        field_errors: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        FinalErrorResponse {
            status_code,
            error_code,
            non_field_errors,
            field_errors,
        }
    }
}
