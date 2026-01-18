use futures::TryFutureExt;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestInit;

use crate::{
    common_data_traits::GetSessionToken,
    compatibility_layer::typescript_wasm::into_input::InputToRequest, endpoint::Endpoint,
    error::ErrorCodes, new_final_error, request_method::RequestMethod,
};

mod into_input;

pub trait Fetchable: Endpoint
where
    Self::OutputStruct: DeserializeOwned,
    Self::InputStruct: InputToRequest,
{
    #[allow(async_fn_in_trait)]
    async fn fetch(
        inputs: <Self as Endpoint>::InputStruct,
    ) -> Result<<Self as Endpoint>::OutputStruct, JsValue> {
        let request = RequestInit::new();
        request.set_method(Self::REQUEST_METHOD.to_str());
        request.set_mode(web_sys::RequestMode::Cors);

        let headers = web_sys::Headers::new().unwrap();

        if <Self as Endpoint>::InputStruct::HAS_SESSION_TOKEN {
            headers.append("Bearer-Token", inputs.get_session_token())?;
        }

        if Self::REQUEST_METHOD != RequestMethod::Get {
            headers.append("Content-Type", "application/json")?;
        }

        let fetch_data = inputs.to_input();

        request.set_headers(&headers);

        if let Some(v) = fetch_data.body {
            let body = serde_wasm_bindgen::to_value(&v).expect("Couldn't turn value to JsValue");
            request.set_body(&body);
        }

        JsFuture::from(
            web_sys::window()
                .expect("Couldn't access window")
                .fetch_with_str_and_init(
                    (Self::construct_full_path() + fetch_data.query_string.as_str()).as_str(),
                    &request,
                ),
        )
        .map_ok(|ok_val| {
            serde_wasm_bindgen::from_value::<<Self as Endpoint>::OutputStruct>(ok_val).map_err(
                |e| {
                    serde_wasm_bindgen::to_value(&new_final_error!(
                        ErrorCodes::SerializingDataToJSON,
                        e
                    ))
                    .expect("This should never be able to fail")
                },
            )
        })
        .await
        .flatten()
    }
}

impl<T> Fetchable for T
where
    T: Endpoint,
    T::OutputStruct: DeserializeOwned,
    T::InputStruct: InputToRequest,
{
}
