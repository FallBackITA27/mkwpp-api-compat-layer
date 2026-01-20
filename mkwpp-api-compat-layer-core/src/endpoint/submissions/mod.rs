
use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{UtcTimestamp, category::Category, submissions::{EditSubmissions, SubmissionStatus, Submissions}, user::UserIdentificationData},
    endpoint::{RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub struct SubmissionsScope;

impl Scope for SubmissionsScope {
    const PATH: &'static str = "/submissions";

    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[internal(path = "/get_list", input = UserIdentificationData, output = Vec<Submissions>, scope = SubmissionsScope, required = RequiredPermission::LoggedIn)]
pub struct GetSubmissionsList;

#[derive(Default, Endpoint)]
#[internal(path = "/get_edit_list", input = UserIdentificationData, output = Vec<EditSubmissions>, scope = SubmissionsScope, required = RequiredPermission::LoggedIn)]
pub struct GetEditSubmissionsList;

#[derive(Default, Endpoint)]
#[internal(path = "/create", input = SubmissionCreation, scope = SubmissionsScope, required = RequiredPermission::LoggedIn)]
pub struct CreateSubmission;

#[derive(Default, Endpoint)]
#[internal(path = "/edit", input = SubmissionEdit, scope = SubmissionsScope, required = RequiredPermission::LoggedIn)]
pub struct EditSubmission;

#[either_field::make_template(
    GenStructs: true,
    DeleteTemplate: false,
    OmitEmptyTupleFields: true;
    pub SubmissionEdit: [
        submission_id: i32
    ]
)]
#[derive(serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct SubmissionCreation {
    #[internal(id)]
    pub submission_id: either_field::either!(() | i32),

    pub value: i32,

    #[internal(category)]
    pub category: Category,

    pub is_lap: bool,

    pub player_id: i32,

    pub track_id: i32,

    pub date: Option<UtcTimestamp>,

    pub video_link: Option<String>,

    pub ghost_link: Option<String>,

    pub comment: Option<String>,

    pub submitter_id: i32,

    pub submitter_note: Option<String>,

    pub admin_note: Option<String>,

    pub reviewer_note: Option<String>,

    pub status: Option<SubmissionStatus>,

    pub reviewer_id: Option<i32>,

    #[internal(session_token)]
    pub session_token: String
}

#[derive(Default, Endpoint)]
#[internal(path = "/create_edit", input = EditSubmissionCreation, scope = SubmissionsScope, required = RequiredPermission::LoggedIn)]
pub struct CreateEditSubmission;

#[derive(Default, Endpoint)]
#[internal(path = "/edit_edit", input = EditSubmissionEdit, scope = SubmissionsScope, required = RequiredPermission::LoggedIn)]
pub struct EditEditSubmission;

#[either_field::make_template(
    GenStructs: true,
    DeleteTemplate: false,
    OmitEmptyTupleFields: true;
    pub EditSubmissionEdit: [
        submission_id: i32
    ]
)]
#[derive(serde::Deserialize, GetId, GetSessionToken, GetCategory)]
pub struct EditSubmissionCreation {
    #[internal(id)]
    pub edit_submission_id: either_field::either!(() | i32),

    pub date: Option<UtcTimestamp>,

    pub video_link: Option<String>,

    pub ghost_link: Option<String>,

    pub comment: Option<String>,

    pub date_edited: bool,

    pub video_link_edited: bool,

    pub ghost_link_edited: bool,

    pub comment_edited: bool,

    pub submitter_id: i32,

    pub submitter_note: Option<String>,

    pub score_id: i32,

    pub admin_note: Option<String>,

    pub reviewer_note: Option<String>,

    pub status: Option<SubmissionStatus>,

    pub reviewer_id: Option<i32>,
}
