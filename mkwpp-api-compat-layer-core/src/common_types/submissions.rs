use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};
use serde::de::Visitor;

use crate::common_types::{UtcTimestamp, category::Category};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SubmissionStatus {
    #[default]
    Pending,
    Accepted,
    Rejected,
    OnHold,
}

impl TryInto<SubmissionStatus> for u8 {
    type Error = ();
    fn try_into(self) -> Result<SubmissionStatus, Self::Error> {
        match self {
            0 => Ok(SubmissionStatus::Pending),
            1 => Ok(SubmissionStatus::Accepted),
            2 => Ok(SubmissionStatus::Rejected),
            3 => Ok(SubmissionStatus::OnHold)
            3..=255 => Err(()),
        }
    }
}

impl From<SubmissionStatus> for u8 {
    fn from(val: SubmissionStatus) -> Self {
        match val {
     SubmissionStatus::Pending       => 0,
     SubmissionStatus::Accepted       => 1,
     SubmissionStatus::Rejected       => 2,
     SubmissionStatus::OnHold       => 3,
        }
    }
}

impl serde::Serialize for SubmissionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8((*self).into())
    }
}

impl<'de> serde::Deserialize<'de> for SubmissionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SubmissionStatusVisitor;
        impl<'de> Visitor<'de> for SubmissionStatusVisitor {
            type Value = SubmissionStatus;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer between 0 and 3")
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

        deserializer.deserialize_u8(SubmissionStatusVisitor)
    }
}

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct Submissions {
    #[internal(id)]
    pub id: i32,

    pub value: i32,

    #[internal(category)]
    pub category: Category

    pub is_lap: bool,

    pub player_id: i32,

    pub track_id: i32,

    pub date: Option<UtcTimestamp>,

    pub video_link: Option<String>,

    pub ghost_link: Option<String>,

    pub comment: Option<String>,

    pub admin_note: Option<String>,

    pub status: SubmissionStatus,

    pub submitter_id: i32,

    pub submitter_note: Option<String>,

    pub submitted_at: UtcTimestamp,

    pub reviewer_id: Option<i32>,
    
    pub reviewer_note: Option<String>,

    pub reviewed_at: Option<UtcTimestamp>,

    pub score_id: Option<i32>,
}

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct EditSubmissions {
    #[internal(id)]
    pub id: i32,

    pub date: Option<UtcTimestamp>,

    pub video_link: Option<String>,

    pub ghost_link: Option<String>,

    pub comment: Option<String>,

    pub date_edited: bool,

    pub video_link_edited: bool,

    pub ghost_link_edited: bool,

    pub comment_edited: bool,

    pub admin_note: Option<String>,

    pub status: SubmissionStatus,

    pub submitter_id: i32,

    pub submitter_note: Option<String>,

    pub submitted_at: UtcTimestamp,

    pub reviewer_id: Option<i32>,

    pub reviewer_note: Option<String>,

    pub reviewed_at: Option<UtcTimestamp>,

    pub score_id: i32,
}
