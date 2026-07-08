use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};
use serde::de::Visitor;

#[either_field::make_template(
    GenStructs: true,
    DeleteTemplate: true,
    OmitEmptyTupleFields: true;
    pub Regions: [ player_count: _ ],
    pub RegionsWithPlayerCount: [ player_count: i32 ],
)]
#[derive(Debug, Clone, GetId, GetSessionToken, GetCategory)]
#[cfg_attr(feature = "rust-actix", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "typescript-wasm", derive(serde::Deserialize))]
pub struct RegionsTemplate {
    #[wasm_bindgen(readonly)]
    #[internal(id)]
    pub id: i32,

    #[wasm_bindgen(readonly)]
    #[wasm_bindgen(getter_with_clone)]
    pub code: String,

    #[wasm_bindgen(readonly)]
    pub region_type: RegionType,

    pub parent_id: Option<i32>,

    #[wasm_bindgen(readonly)]
    pub is_ranked: bool,

    #[wasm_bindgen(readonly)]
    pub player_count: either_field::either!(() | i32),
}

#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub enum RegionType {
    #[default]
    World,
    Continent,
    CountryGroup,
    Country,
    SubnationalGroup,
    Subnational,
}

impl TryInto<RegionType> for u8 {
    type Error = ();
    fn try_into(self) -> Result<RegionType, Self::Error> {
        match self {
            0 => Ok(RegionType::World),
            1 => Ok(RegionType::Continent),
            2 => Ok(RegionType::CountryGroup),
            3 => Ok(RegionType::Country),
            4 => Ok(RegionType::SubnationalGroup),
            5 => Ok(RegionType::Subnational),
            6..=255 => Err(()),
        }
    }
}

impl From<RegionType> for u8 {
    fn from(val: RegionType) -> Self {
        match val {
            RegionType::World => 0,
            RegionType::Continent => 1,
            RegionType::CountryGroup => 2,
            RegionType::Country => 3,
            RegionType::SubnationalGroup => 4,
            RegionType::Subnational => 5,
        }
    }
}

impl serde::Serialize for RegionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8((*self).into())
    }
}

impl<'de> serde::Deserialize<'de> for RegionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RegionTypeVisitor;
        impl<'de> Visitor<'de> for RegionTypeVisitor {
            type Value = RegionType;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer between 0 and 5")
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

        deserializer.deserialize_u8(RegionTypeVisitor)
    }
}
