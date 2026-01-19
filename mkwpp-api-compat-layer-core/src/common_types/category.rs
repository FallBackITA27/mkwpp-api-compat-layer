use serde::de::Visitor;

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

