use std::collections::HashMap;

use crate::status_code::StatusCode;

#[cfg_attr(feature = "rust-actix", derive(Debug, serde::Serialize))]
pub struct FinalErrorResponse {
    #[cfg_attr(feature = "rust-actix", serde(skip))]
    pub status_code: StatusCode,
    pub error_code: u64,
    pub non_field_errors: Vec<String>,
    pub field_errors: std::collections::HashMap<String, Vec<String>>,
}

impl FinalErrorResponse {
    pub fn new(
        error_code: u64,
        status_code: StatusCode,
        non_field_errors: Vec<String>,
        field_errors: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        FinalErrorResponse {
            status_code,
            error_code,
            non_field_errors,
            field_errors,
        }
    }
}

pub enum ErrorCodes {
    NoConnectionFromPGPool,
    SerializingDataToJSON,
    ClosingConnectionFromPGPool,
    GettingFromDatabase,
    DecodingDatabaseRows,
    UserIdToPlayerId,
    GenerateTimesheet,
    GenerateMatchup,
    UsernameTooShort,
    UsernameTooLong,
    PasswordTooLong,
    PasswordTooShort,
    PasswordMustHaveSpecial,
    PasswordMustHaveLowercase,
    PasswordMustHaveUppercase,
    PasswordMustHaveNumber,
    EmailTooLong,
    EmailInvalid,
    UserIDDoesntExist,
    InvalidSessionToken,
    UserHasNoAssociatedPlayer,
    CreatePGTransaction,
    CommitPGTransaction,
    InsufficientPermissions,
    GeneratingToken,
    MismatchedIds,
    NothingChanged,
    InvalidInput,
    TechnicallyUnreachableCode,
    CreatingEmailClient,
    SendingEmail,
    UserNotVerified,
    UserOnCooldown,
    NoAssociatedPlayer,
    InvalidChadsoftID,
    NoDataToSerialize,
    CannotReadFile,
    RollBackPGTransaction,
}

impl From<ErrorCodes> for u64 {
    fn from(val: ErrorCodes) -> Self {
        match val {
            ErrorCodes::NoConnectionFromPGPool => 0,
            ErrorCodes::SerializingDataToJSON => 1,
            ErrorCodes::ClosingConnectionFromPGPool => 2,
            ErrorCodes::GettingFromDatabase => 3,
            ErrorCodes::DecodingDatabaseRows => 4,
            ErrorCodes::UserIdToPlayerId => 5,
            ErrorCodes::GenerateTimesheet => 6,
            ErrorCodes::GenerateMatchup => 7,
            ErrorCodes::UsernameTooShort => 8,
            ErrorCodes::UsernameTooLong => 9,
            ErrorCodes::PasswordTooLong => 10,
            ErrorCodes::PasswordTooShort => 11,
            ErrorCodes::PasswordMustHaveSpecial => 12,
            ErrorCodes::PasswordMustHaveLowercase => 13,
            ErrorCodes::PasswordMustHaveUppercase => 14,
            ErrorCodes::PasswordMustHaveNumber => 15,
            ErrorCodes::EmailTooLong => 16,
            ErrorCodes::EmailInvalid => 17,
            ErrorCodes::UserIDDoesntExist => 18,
            ErrorCodes::InvalidSessionToken => 19,
            ErrorCodes::UserHasNoAssociatedPlayer => 20,
            ErrorCodes::CreatePGTransaction => 21,
            ErrorCodes::CommitPGTransaction => 22,
            ErrorCodes::InsufficientPermissions => 23,
            ErrorCodes::GeneratingToken => 24,
            ErrorCodes::MismatchedIds => 25,
            ErrorCodes::NothingChanged => 26,
            ErrorCodes::InvalidInput => 27,
            ErrorCodes::TechnicallyUnreachableCode => 28,
            ErrorCodes::CreatingEmailClient => 29,
            ErrorCodes::SendingEmail => 30,
            ErrorCodes::UserNotVerified => 31,
            ErrorCodes::UserOnCooldown => 32,
            ErrorCodes::NoAssociatedPlayer => 33,
            ErrorCodes::InvalidChadsoftID => 34,
            ErrorCodes::NoDataToSerialize => 35,
            ErrorCodes::CannotReadFile => 36,
            ErrorCodes::RollBackPGTransaction => 37,
        }
    }
}

impl ErrorCodes {
    pub fn into_final_error(self, library_error: impl ToString) -> FinalErrorResponse {
        let mut out = match self {
            Self::NoConnectionFromPGPool => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Couldn't get connection from data pool")],
                HashMap::new(),
            ),
            Self::SerializingDataToJSON => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error serializing data to JSON")],
                HashMap::new(),
            ),
            Self::ClosingConnectionFromPGPool => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error closing Database connection")],
                HashMap::new(),
            ),
            Self::GettingFromDatabase => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Couldn't get rows from database")],
                HashMap::new(),
            ),
            Self::DecodingDatabaseRows => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error decoding database rows")],
                HashMap::new(),
            ),
            Self::UserIdToPlayerId => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error converting User ID to Player ID")],
                HashMap::new(),
            ),
            Self::GenerateTimesheet => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error generating timesheet")],
                HashMap::new(),
            ),
            Self::GenerateMatchup => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error generating matchup")],
                HashMap::new(),
            ),

            Self::UsernameTooShort => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the username")],
                std::collections::HashMap::from([(
                    String::from("username"),
                    vec![String::from("Username too short")],
                )]),
            ),
            Self::UsernameTooLong => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the username")],
                std::collections::HashMap::from([(
                    String::from("username"),
                    vec![String::from("Username too long")],
                )]),
            ),
            Self::PasswordTooLong => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the password")],
                std::collections::HashMap::from([(
                    String::from("password"),
                    vec![String::from("Password too long")],
                )]),
            ),
            Self::PasswordTooShort => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the password")],
                std::collections::HashMap::from([(
                    String::from("password"),
                    vec![String::from("Password too short")],
                )]),
            ),
            Self::PasswordMustHaveSpecial => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the password")],
                std::collections::HashMap::from([(
                    String::from("password"),
                    vec![String::from("Password must have a special character")],
                )]),
            ),
            Self::PasswordMustHaveLowercase => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the password")],
                std::collections::HashMap::from([(
                    String::from("password"),
                    vec![String::from("Password must have a lowercase character")],
                )]),
            ),
            Self::PasswordMustHaveUppercase => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the password")],
                std::collections::HashMap::from([(
                    String::from("password"),
                    vec![String::from("Password must have an uppercase character")],
                )]),
            ),
            Self::PasswordMustHaveNumber => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the password")],
                std::collections::HashMap::from([(
                    String::from("password"),
                    vec![String::from("Password must have a number")],
                )]),
            ),
            Self::EmailTooLong => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the email")],
                std::collections::HashMap::from([(
                    String::from("email"),
                    vec![String::from("Email too long")],
                )]),
            ),
            Self::EmailInvalid => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Error validating the email")],
                std::collections::HashMap::from([(
                    String::from("email"),
                    vec![String::from("Email invalid")],
                )]),
            ),
            Self::UserIDDoesntExist => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error getting user ID")],
                HashMap::new(),
            ),

            Self::InvalidSessionToken => FinalErrorResponse::new(
                self.into(),
                StatusCode::Forbidden,
                vec![String::from("Invalid session token")],
                HashMap::new(),
            ),
            Self::UserHasNoAssociatedPlayer => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("User has no associated player")],
                HashMap::new(),
            ),
            Self::CreatePGTransaction => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error creating postgres transaction")],
                HashMap::new(),
            ),
            Self::CommitPGTransaction => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error committing postgres transaction")],
                HashMap::new(),
            ),
            Self::InsufficientPermissions => FinalErrorResponse::new(
                self.into(),
                StatusCode::Forbidden,
                vec![String::from("Insufficient Permissions")],
                HashMap::new(),
            ),
            Self::GeneratingToken => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error generating token")],
                HashMap::new(),
            ),
            Self::MismatchedIds => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Mismatched IDs")],
                HashMap::new(),
            ),
            Self::NothingChanged => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Nothing to update")],
                HashMap::new(),
            ),
            Self::InvalidInput => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Input is invalid")],
                HashMap::new(),
            ),
            Self::TechnicallyUnreachableCode => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from(
                    "Technically unreachable code has been reached",
                )],
                HashMap::new(),
            ),
            Self::CreatingEmailClient => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("There was an error creating the email client")],
                HashMap::new(),
            ),
            Self::SendingEmail => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("There was an error sending the email")],
                HashMap::new(),
            ),
            Self::UserNotVerified => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("User is not verified")],
                HashMap::new(),
            ),
            Self::UserOnCooldown => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("User is on cooldown")],
                HashMap::new(),
            ),
            Self::NoAssociatedPlayer => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("There is no associated player")],
                HashMap::new(),
            ),
            Self::InvalidChadsoftID => FinalErrorResponse::new(
                self.into(),
                StatusCode::BadRequest,
                vec![String::from("Chadsoft ID is not valid")],
                HashMap::new(),
            ),
            Self::NoDataToSerialize => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Whoever coded this is a moron")],
                HashMap::new(),
            ),
            Self::CannotReadFile => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Could not read file")],
                HashMap::new(),
            ),
            Self::RollBackPGTransaction => FinalErrorResponse::new(
                self.into(),
                StatusCode::InternalServerError,
                vec![String::from("Error rolling back postgres transaction")],
                HashMap::new(),
            ),
        };

        let library_error = library_error.to_string();
        if !library_error.is_empty() {
            out.non_field_errors.push(library_error);
        }

        out
    }
}
