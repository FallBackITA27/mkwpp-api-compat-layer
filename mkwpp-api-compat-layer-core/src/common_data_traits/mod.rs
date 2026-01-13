use crate::common_types::{Category, NoData};

/// Marker Type
pub trait DataTraits: GetId + GetSessionToken + GetCategory {}
impl<T> DataTraits for T where T: GetId + GetSessionToken + GetCategory {}

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

/// Item category
pub trait HasCategory {
    fn get_category(&self) -> Category;
}

pub trait GetCategory {
    const HAS_CATEGORY: bool = false;
    fn get_category(&self) -> Category {
        Category::Normal
    }
}

impl<T> GetCategory for T
where
    T: HasCategory,
{
    const HAS_CATEGORY: bool = true;
    fn get_category(&self) -> Category {
        HasCategory::get_category(self)
    }
}

// NoData impls
impl GetId for NoData {}
impl GetSessionToken for NoData {}
impl GetCategory for NoData {}
