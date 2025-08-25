
/// Marker Type
pub trait DataTraits: GetId + GetSessionToken {}

pub trait GetId {
    const HAS_ID: bool;
    fn get_id(&self) -> i32;
}

pub trait GetSessionToken {
    const HAS_SESSION_TOKEN: bool;
    fn get_token(&self) -> &str;
}

// The () impls below here

impl DataTraits for () {}

impl GetId for () {
    const HAS_ID: bool = false;
    fn get_id(&self) -> i32 {
        Default::default()
    }
}

impl GetSessionToken for () {
    const HAS_SESSION_TOKEN: bool = false;
    fn get_token(&self) -> &str {
        Default::default()
    }
}