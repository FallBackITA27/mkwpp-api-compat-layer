use mkwpp_api_compat_layer_macros::FromIntoInner;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, FromIntoInner, Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct AverageFinish(f64);

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, FromIntoInner, Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct AverageRankRating(f64);

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, FromIntoInner, Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct PersonalRecordWorldRecord(f64);

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, FromIntoInner, Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct TallyPoints(i16);

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, FromIntoInner, Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct TotalTime(i32);
