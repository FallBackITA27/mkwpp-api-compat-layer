use actix_web::{FromRequest, HttpResponse, Route, web};

use crate::{
    common_data_traits::GetSessionToken,
    endpoint::Endpoint,
    error::{ErrorCodes, FinalErrorResponse, PPResult},
    required_permission::RequiredPermission,
    status_code::StatusCode,
};

pub trait ToActixRoute: Endpoint
where
    Self::InputStruct: FromRequest + 'static,
    Self::OutputStruct: serde::Serialize,
{
    /// Only call this once, please.
    fn to_actix_route(
        handler: impl AsyncFn(Self::InputStruct) -> PPResult<Self::OutputStruct> + 'static,
    ) -> Route {
        let handler = std::sync::Arc::new(handler);

        let inner_handler = move |input: Self::InputStruct| {
            let handler = handler.clone();
            async move {
                if Self::REQUIRED_PERMISSION == RequiredPermission::None
                    && !Self::InputStruct::HAS_SESSION_TOKEN
                {
                    // If fail here then something is wrong in the endpoints
                    // lol need to do None::<&str>
                    return Err(ErrorCodes::InsufficientPermissions
                        .into_final_error(None::<&str>, file!(), line!())
                        .into_response(StatusCode::Forbidden));
                }

                let data = handler(input).await?;
                Ok::<HttpResponse, FinalErrorResponse>(HttpResponse::Ok().json(data))
            }
        };

        web::method(Self::REQUEST_METHOD.into()).to(inner_handler)
    }
}

impl<T> ToActixRoute for T
where
    T: Endpoint,
    Self::InputStruct: FromRequest + 'static,
    T::OutputStruct: serde::Serialize,
{
}
