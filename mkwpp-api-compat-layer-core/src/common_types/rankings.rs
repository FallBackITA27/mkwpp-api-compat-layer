#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct AverageFinish(f64);
impl From<f64> for AverageFinish {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl From<AverageFinish> for f64 {
    fn from(value: AverageFinish) -> f64 {
        value.0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct AverageRankRating(f64);
impl From<f64> for AverageRankRating {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl From<AverageRankRating> for f64 {
    fn from(value: AverageRankRating) -> f64 {
        value.0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct PersonalRecordWorldRecord(f64);
impl From<f64> for PersonalRecordWorldRecord {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl From<PersonalRecordWorldRecord> for f64 {
    fn from(value: PersonalRecordWorldRecord) -> f64 {
        value.0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct TallyPoints(i16);
impl From<i16> for TallyPoints {
    fn from(value: i16) -> Self {
        Self(value)
    }
}
impl From<TallyPoints> for i16 {
    fn from(value: TallyPoints) -> i16 {
        value.0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct TotalTime(i32);
impl From<i32> for TotalTime {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl From<TotalTime> for i32 {
    fn from(value: TotalTime) -> i32 {
        value.0
    }
}
