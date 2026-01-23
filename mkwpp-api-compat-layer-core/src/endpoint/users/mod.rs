use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        UtcTimestamp,
        category::Category,
        user::{
            ClientSideUserData, Email, Password, UserIdentificationData, UserLoginData,
            UserRegisterData,
        },
    },
    endpoint::{RequiredPermission, Root, Scope},
    request_method::RequestMethod,
};

pub struct UsersScope;

impl Scope for UsersScope {
    const PATH: &'static str = "/users";

    type OuterScope = Root;
}

#[derive(Default, Endpoint)]
#[internal(path = "/register", input = UserRegisterData, scope = UsersScope)]
pub struct RegisterUser;

#[derive(Default, Endpoint)]
#[internal(path = "/login", input = UserLoginData, output = LoginUserOutput, scope = UsersScope)]
pub struct LoginUser;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct LoginUserOutput {
    #[internal(session_token)]
    session_token: String,

    expiry: UtcTimestamp,
}

#[derive(Default, Endpoint)]
#[internal(path = "/logout", input = UserIdentificationData, scope = UsersScope, required = RequiredPermission::LoggedIn)]
pub struct LogoutUser;

#[derive(Default, Endpoint)]
#[internal(path = "/activate", input = ActivateUserInput, scope = UsersScope)]
pub struct ActivateUser;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct ActivateUserInput {
    token: String,
}

#[derive(Default, Endpoint)]
#[internal(path = "/get_user_data", input = UserIdentificationData, output = ClientSideUserData, scope = UsersScope, required = RequiredPermission::LoggedIn)]
pub struct GetUser;

#[derive(Default, Endpoint)]
#[internal(path = "/password_forgot", input = PasswordForgotInput, scope = UsersScope)]
pub struct PasswordForgot;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct PasswordForgotInput {
    email: Email,
}

#[derive(Default, Endpoint)]
#[internal(path = "/password_reset", input = PasswordResetInput, scope = UsersScope)]
pub struct PasswordReset;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct PasswordResetInput {
    password: Password,
    token: String,
}

#[derive(Default, Endpoint)]
#[internal(path = "/password_reset_token_check", input = PasswordResetTokenCheckInput, output = PasswordResetTokenCheckOutput, scope = UsersScope)]
pub struct PasswordResetTokenCheck;

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct PasswordResetTokenCheckInput {
    token: String,
}

#[derive(GetId, GetCategory, GetSessionToken)]
pub struct PasswordResetTokenCheckOutput {
    is_valid: bool,
}
