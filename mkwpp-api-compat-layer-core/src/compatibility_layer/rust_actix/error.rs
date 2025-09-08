use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};

use crate::{error::FinalErrorResponse, status_code::StatusCode};

impl FinalErrorResponse {
    fn generate_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(actix_web::http::StatusCode::from(self.status_code)).json(self)
    }
}

impl ResponseError for FinalErrorResponse {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        self.generate_response()
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from(self.status_code)
    }
}

impl From<StatusCode> for actix_web::http::StatusCode {
    fn from(value: StatusCode) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(value.to_number()).unwrap()
    }
}

impl std::fmt::Display for FinalErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FinalErrorResponse: {{ errors: {:?}, status_code: {:?} }}",
            self.errors, self.status_code
        )
    }
}
