use mkwpp_api_compat_layer_macros::{GetCategory, GetId, GetSessionToken};

use crate::error::ErrorCodes;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct UserIdentificationData {
    #[internal(session_token)]
    session_token: String,
}

#[derive(serde::Serialize, GetId, GetCategory, GetSessionToken)]
pub struct ClientSideUserData {
    pub player_id: Option<i32>,
    pub user_id: i32,
    pub username: Username,
}

#[either_field::make_template(
    GenStructs: true,
    DeleteTemplate: false,
    OmitEmptyTupleFields: true;
    pub UserLoginData: [
        email: ()
    ]
)]
#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct UserRegisterData {
    username: Username,
    password: Password,
    email: either_field::either!(Email | ()),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = ErrorCodes;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len() {
            0..=3 => Err(ErrorCodes::UsernameTooShort),
            151.. => Err(ErrorCodes::UsernameTooLong),
            _ => Ok(Self(value)),
        }
    }
}

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = ErrorCodes;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 254 {
            return Err(ErrorCodes::EmailTooLong);
        }

        let regex_checker = regex::Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
        ).unwrap();

        match regex_checker.is_match(&value) {
            true => Ok(Self(value)),
            false => Err(ErrorCodes::EmailInvalid),
        }
    }
}

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.0
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Password(String);

impl TryFrom<String> for Password {
    type Error = ErrorCodes;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = match value.len() {
            0..=8 => return Err(ErrorCodes::PasswordTooShort),
            129.. => return Err(ErrorCodes::PasswordTooLong),
            _ => value,
        };

        let mut has_uppercase = false;
        let mut has_lowercase = false;
        let mut has_special_character = false;
        let mut has_number = false;
        for character in value.chars() {
            if !character.is_alphanumeric() {
                has_special_character = true;
                continue;
            }

            if character.is_numeric() {
                has_number = true;
                continue;
            }

            if character.is_lowercase() {
                has_lowercase = true;
                continue;
            }

            if character.is_uppercase() {
                has_uppercase = true;
                continue;
            }
        }

        if !has_uppercase {
            return Err(ErrorCodes::PasswordMustHaveUppercase);
        }
        if !has_lowercase {
            return Err(ErrorCodes::PasswordMustHaveLowercase);
        }
        if !has_special_character {
            return Err(ErrorCodes::PasswordMustHaveSpecial);
        }
        if !has_number {
            return Err(ErrorCodes::PasswordMustHaveNumber);
        }

        Ok(Self(value))
    }
}

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}
