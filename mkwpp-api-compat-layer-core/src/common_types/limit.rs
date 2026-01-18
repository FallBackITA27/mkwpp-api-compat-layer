#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Limit(i32);

impl From<Limit> for i32 {
    fn from(value: Limit) -> Self {
        value.0
    }
}

impl From<i32> for Limit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Default for Limit {
    fn default() -> Self {
        Self(i32::MAX)
    }
}
