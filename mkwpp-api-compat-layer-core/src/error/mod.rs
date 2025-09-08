use crate::status_code::StatusCode;

pub type PPResult<T> = Result<T, FinalErrorResponse>;

#[cfg_attr(feature = "rust-actix", derive(serde::Serialize, Debug))]
pub struct FinalErrorResponse {
    pub status_code: StatusCode,
    pub errors: Vec<FinalError>,
}

impl FinalErrorResponse {
    #[inline]
    pub fn new(status_code: StatusCode, errors: Vec<FinalError>) -> FinalErrorResponse {
        FinalErrorResponse {
            status_code,
            errors,
        }
    }

    #[inline]
    pub fn new_from_final_error(status_code: StatusCode, error: FinalError) -> FinalErrorResponse {
        FinalErrorResponse {
            status_code,
            errors: vec![error],
        }
    }

    #[inline]
    pub fn push(&mut self, error: FinalError) {
        self.errors.push(error);
    }
}

#[cfg_attr(feature = "rust-actix", derive(Debug))]
#[cfg_attr(
    any(feature = "typescript-wasm", feature = "rust-actix"),
    derive(serde::Serialize)
)]
pub struct FinalError {
    pub error_code: u64,

    pub field: Option<&'static str>,
    pub error_text: &'static str,

    pub library_error: Option<String>,

    pub backend_file: &'static str,
    pub backend_line: u32,
}

impl FinalError {
    fn new(
        error_code: u64,
        field: Option<&'static str>,
        error_text: &'static str,
        library_error: Option<impl ToString>,
        backend_file: &'static str,
        backend_line: u32,
    ) -> Self {
        Self {
            error_code,
            field,
            error_text,
            backend_file,
            backend_line,
            library_error: library_error.map(|r| r.to_string()),
        }
    }

    #[inline]
    pub fn into_response(self, status_code: StatusCode) -> FinalErrorResponse {
        FinalErrorResponse::new_from_final_error(status_code, self)
    }
}

#[macro_export]
macro_rules! new_final_error {
    ($error_code: path) => {
        ErrorCodes::into_final_error($error_code, None, file!(), line!())
    };

    ($error_code: path, $impl_to_str: tt) => {
        ErrorCodes::into_final_error($error_code, Some($impl_to_str), file!(), line!())
    };
}

#[derive(Clone, Copy)]
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
    pub fn into_final_error(
        self,
        library_error: Option<impl ToString>,
        backend_file: &'static str,
        backend_line: u32,
    ) -> FinalError {
        let (field, error_text) = match self {
            Self::NoConnectionFromPGPool => (None, "Couldn't get connection from data pool"),
            Self::SerializingDataToJSON => (None, "Error serializing data to JSON"),
            Self::ClosingConnectionFromPGPool => (None, "Error closing Database connection"),
            Self::GettingFromDatabase => (None, "Couldn't get rows from database"),
            Self::DecodingDatabaseRows => (None, "Error decoding database rows"),
            Self::UserIdToPlayerId => (None, "Error converting User ID to Player ID"),
            Self::GenerateTimesheet => (None, "Error generating timesheet"),
            Self::GenerateMatchup => (None, "Error generating matchup"),
            Self::UsernameTooShort => (Some("username"), "Username too short"),
            Self::UsernameTooLong => (Some("username"), "Username too long"),
            Self::PasswordTooLong => (Some("password"), "Password too long"),
            Self::PasswordTooShort => (Some("password"), "Password too short"),
            Self::PasswordMustHaveSpecial => {
                (Some("password"), "Password must have a special character")
            }
            Self::PasswordMustHaveLowercase => {
                (Some("password"), "Password must have a lowercase character")
            }
            Self::PasswordMustHaveUppercase => {
                (Some("password"), "Password must have a uppercase character")
            }
            Self::PasswordMustHaveNumber => (Some("password"), "Password must have a number"),
            Self::EmailTooLong => (Some("email"), "Email too long"),
            Self::EmailInvalid => (Some("email"), "Email invalid"),
            Self::UserIDDoesntExist => (None, "Error getting user ID"),
            Self::InvalidSessionToken => (None, "Invalid session token"),
            Self::UserHasNoAssociatedPlayer => (None, "User has no associated player profile"),
            Self::CreatePGTransaction => (None, "Error creating postgres transaction"),
            Self::CommitPGTransaction => (None, "Error committing postgres transaction"),
            Self::InsufficientPermissions => (None, "Insufficient permissions"),
            Self::GeneratingToken => (None, "Error generating token"),
            Self::MismatchedIds => (None, "Mismatched IDs"),
            Self::NothingChanged => (None, "Nothing to update"),
            Self::InvalidInput => (None, "Input is invalid"),
            Self::TechnicallyUnreachableCode => {
                (None, "Technically unreachable code has been reached")
            }
            Self::CreatingEmailClient => (None, "There was an error creating the email client"),
            Self::SendingEmail => (None, "There was an error sending the email"),
            Self::UserNotVerified => (None, "User is not verified"),
            Self::UserOnCooldown => (None, "User is on cooldown"),
            Self::NoAssociatedPlayer => (None, "There is no associated player"),
            Self::InvalidChadsoftID => (None, "Chadsoft ID is not valid"),
            Self::NoDataToSerialize => (None, "Whoever coded this is a moron"),
            Self::CannotReadFile => (None, "Could not read file"),
            Self::RollBackPGTransaction => (None, "Error rolling back postgres transaction"),
        };

        FinalError::new(
            self.into(),
            field,
            error_text,
            library_error,
            backend_file,
            backend_line,
        )
    }
}
