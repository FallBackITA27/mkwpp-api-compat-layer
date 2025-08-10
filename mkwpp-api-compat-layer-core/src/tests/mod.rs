#[cfg(feature = "rust-actix")]
mod rust_actix {
    use strum::IntoEnumIterator;

    use crate::status_code::StatusCode;

    #[test]
    fn status_code_conversion_works() {
        // Fails if From panics on unwrap
        for option in StatusCode::iter() {
            let _ = actix_web::http::StatusCode::from(option);
        }
    }
}
