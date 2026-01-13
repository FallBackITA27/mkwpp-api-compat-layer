use serde::de::Visitor;

pub mod players;
pub mod rankings;
pub mod regions;

#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone, Copy)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Filter(i32);

impl From<Filter> for i32 {
    fn from(value: Filter) -> Self {
        value.0
    }
}

impl From<i32> for Filter {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct UtcTimestamp(f64);

#[derive(Clone, Copy)]
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
        serializer.serialize_str(format!("{:016X}", self.0))
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
                write!(formatter, "an integer between 0 and 2")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let v = u64::from_str_radix(v, 16)
                    .map_err(|_| serde::de::Error::custom("Could not convert timestamp to date"))?;
                Ok(v as i64)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ChadsoftID(v))
            }
        }
    }
}

#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct NoData;
