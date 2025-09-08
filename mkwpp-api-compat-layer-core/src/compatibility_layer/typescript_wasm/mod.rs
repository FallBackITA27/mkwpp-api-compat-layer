
use futures::TryFutureExt;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestInit;

use crate::{
    common_data_traits::GetSessionToken,
    endpoint::Endpoint,
    error::ErrorCodes,
    new_final_error,
    request_method::RequestMethod,
};

pub trait Fetchable: Endpoint
where
    Self::OutputStruct: DeserializeOwned,
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
            headers.append("Bearer-Token", inputs.get_token())?;
        }

        if Self::REQUEST_METHOD != RequestMethod::Get {
            headers.append("Content-Type", "application/json")?;
        }

        request.set_headers(&headers);

        JsFuture::from(
            web_sys::window()
                .expect("Couldn't access window")
                .fetch_with_str_and_init(Self::construct_full_path().as_str(), &request),
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
{
}
