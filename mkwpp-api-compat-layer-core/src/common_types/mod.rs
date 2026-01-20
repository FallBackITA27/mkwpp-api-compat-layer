use mkwpp_api_compat_layer_macros::FromIntoInner;
use serde::de::Visitor;

pub mod category;
pub mod submissions;
pub mod lap_mode;
pub mod limit;
pub mod players;
pub mod rankings;
pub mod regions;
pub mod scores;
pub mod user;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, FromIntoInner)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct UtcTimestamp(f64);

#[derive(Debug, Clone, Copy, FromIntoInner)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct ChadsoftID(i64);

impl serde::Serialize for ChadsoftID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:016X}", self.0))
    }
}

impl<'de> serde::Deserialize<'de> for ChadsoftID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ChadsoftIDVisitor;
        impl<'de> Visitor<'de> for ChadsoftIDVisitor {
            type Value = ChadsoftID;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "An i64 or Hex String")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let v = u64::from_str_radix(v, 16)
                    .map_err(|_| serde::de::Error::custom("Could not convert to chadsoft id"))?;
                Ok(ChadsoftID(v as i64))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ChadsoftID(v))
            }
        }

        match deserializer.is_human_readable() {
            true => deserializer.deserialize_str(ChadsoftIDVisitor),
            false => deserializer.deserialize_i64(ChadsoftIDVisitor),
        }
    }
}

#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct NoData;
