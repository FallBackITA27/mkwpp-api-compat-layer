use serde::de::Visitor;

#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Copy, Default)]
pub enum LapMode {
    #[default]
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
