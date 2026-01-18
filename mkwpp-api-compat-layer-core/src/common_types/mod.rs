use serde::de::Visitor;

pub mod limit;
pub mod players;
pub mod rankings;
pub mod regions;
pub mod scores;

#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub enum Category {
    Normal,
    Shortcut,
    Unrestricted,
}

impl TryInto<Category> for u8 {
    type Error = ();
    fn try_into(self) -> Result<Category, Self::Error> {
        match self {
            0 => Ok(Category::Normal),
            1 => Ok(Category::Shortcut),
            2 => Ok(Category::Unrestricted),
            3..=255 => Err(()),
        }
    }
}

impl From<Category> for u8 {
    fn from(val: Category) -> Self {
        match val {
            Category::Normal => 0,
            Category::Shortcut => 1,
            Category::Unrestricted => 2,
        }
    }
}

impl serde::Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8((*self).into())
    }
}

impl<'de> serde::Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CategoryVisitor;
        impl<'de> Visitor<'de> for CategoryVisitor {
            type Value = Category;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer between 0 and 2")
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.try_into().map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(v as u64),
                        &self,
                    )
                })
            }
        }

        deserializer.deserialize_u8(CategoryVisitor)
    }
}

#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub enum LapMode {
    Course,
    FastLap,
    Overall,
}

impl TryInto<LapMode> for u8 {
    type Error = ();
    fn try_into(self) -> Result<LapMode, Self::Error> {
        match self {
            0 => Ok(LapMode::Course),
            1 => Ok(LapMode::FastLap),
            2 => Ok(LapMode::Overall),
            3..=255 => Err(()),
        }
    }
}

impl From<LapMode> for u8 {
    fn from(val: LapMode) -> Self {
        match val {
            LapMode::Course => 0,
            LapMode::FastLap => 1,
            LapMode::Overall => 2,
        }
    }
}

impl serde::Serialize for LapMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8((*self).into())
    }
}

impl<'de> serde::Deserialize<'de> for LapMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LapModeVisitor;
        impl<'de> Visitor<'de> for LapModeVisitor {
            type Value = LapMode;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer between 0 and 2")
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.try_into().map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(v as u64),
                        &self,
                    )
                })
            }
        }

        deserializer.deserialize_u8(LapModeVisitor)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct UtcTimestamp(f64);

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct ChadsoftID(i64);

impl From<ChadsoftID> for i64 {
    fn from(value: ChadsoftID) -> Self {
        value.0
    }
}

impl From<i64> for ChadsoftID {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

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
