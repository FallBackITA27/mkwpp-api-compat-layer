use crate::common_types::NoData;

/// Marker Type
pub trait DataTraits: GetId + GetSessionToken {}
impl<T> DataTraits for T where T: GetId + GetSessionToken {}

// Item ID
pub trait HasId {
    fn get_id(&self) -> i32;
}

pub trait GetId {
    const HAS_ID: bool = false;
    fn get_id(&self) -> i32 {
        Default::default()
    }
}

impl<T> GetId for T
where
    T: HasId,
{
    const HAS_ID: bool = true;
    fn get_id(&self) -> i32 {
        HasId::get_id(self)
    }
}

// Item session Token

pub trait HasSessionToken {
    fn get_token(&self) -> &str;
}

pub trait GetSessionToken {
    const HAS_SESSION_TOKEN: bool = false;
    fn get_token(&self) -> &str {
        Default::default()
    }
}

impl<T> GetSessionToken for T
where
    T: HasSessionToken,
{
    const HAS_SESSION_TOKEN: bool = true;
    fn get_token(&self) -> &str {
        HasSessionToken::get_token(self)
    }
}

// The NoData impls below here

impl GetId for NoData {}
impl GetSessionToken for NoData {}
