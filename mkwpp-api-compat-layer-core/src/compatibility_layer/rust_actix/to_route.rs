use actix_web::{HttpRequest, HttpResponse, Route, web};

use crate::{
    common_data_traits::GetSessionToken,
    endpoint::Endpoint,
    error::{ErrorCodes, FinalErrorResponse, PPResult},
    required_permission::RequiredPermission,
    status_code::StatusCode,
};

pub struct ExtraInput {
    pub ip: std::net::IpAddr
}

pub trait ToActixRoute: Endpoint
where
    Self::InputStruct: serde::de::DeserializeOwned + 'static,
    Self::OutputStruct: serde::Serialize,
{
    /// Only call this once, please.
    fn to_actix_route(
        handler: impl AsyncFn(Self::InputStruct, ExtraInput) -> PPResult<Self::OutputStruct> + 'static,
    ) -> Route {
        let handler = std::sync::Arc::new(handler);

        let inner_handler = move |req: HttpRequest, data: Option<web::Json<Self::InputStruct>>| {
            let handler = handler.clone();
            async move {
                // TODO: better code for getting inputs
                let input: Self::InputStruct = match serde_urlencoded::from_str(req.query_string())
                {
                    Ok(v) => v,
                    Err(e1) => match data {
                        Some(v) => v.into_inner(),
                        None => {
                            return Err(ErrorCodes::InvalidInput
                                .into_final_error(Some(format!("{e1}")), file!(), line!())
                                .into_response(StatusCode::BadRequest));
                        }
                    },
                };

                let extra_input = ExtraInput {
                    ip: req.peer_addr().unwrap().ip()
                };

                if Self::REQUIRED_PERMISSION != RequiredPermission::None
                    && !Self::InputStruct::HAS_SESSION_TOKEN
                {
                    // If fail here then something is wrong in the endpoints
                    return Err(ErrorCodes::InsufficientPermissions
                        .into_final_error(None::<&str>, file!(), line!())
                        .into_response(StatusCode::Forbidden));
                }

                let data = handler(input, extra_input).await?;
                Ok::<HttpResponse, FinalErrorResponse>(HttpResponse::Ok().json(data))
            }
        };

        web::method(Self::REQUEST_METHOD.into()).to(inner_handler)
    }
}

impl<T> ToActixRoute for T
where
    T: Endpoint,
    T::InputStruct: serde::de::DeserializeOwned + 'static,
    T::OutputStruct: serde::Serialize,
{
}
