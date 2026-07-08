use mkwpp_api_compat_layer_macros::{Endpoint, GetCategory, GetId, GetSessionToken};

use crate::{
    common_types::{
        category::Category,
        user::{
            ClientSideUserData, Email, Password, UserIdentificationData, UserLoginData,
            UserRegisterData,
        },
        utc_timestamp::UtcTimestamp,
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

#[derive(serde::Serialize, GetId, GetCategory, GetSessionToken)]
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

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct ActivateUserInput {
    token: String,
}

#[derive(Default, Endpoint)]
#[internal(path = "/get_user_data", input = UserIdentificationData, output = ClientSideUserData, scope = UsersScope, required = RequiredPermission::LoggedIn)]
pub struct GetUser;

#[derive(Default, Endpoint)]
#[internal(path = "/password_forgot", input = PasswordForgotInput, scope = UsersScope)]
pub struct PasswordForgot;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct PasswordForgotInput {
    email: Email,
}

#[derive(Default, Endpoint)]
#[internal(path = "/password_reset", input = PasswordResetInput, scope = UsersScope)]
pub struct PasswordReset;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct PasswordResetInput {
    password: Password,
    token: String,
}

#[derive(Default, Endpoint)]
#[internal(path = "/password_reset_token_check", input = PasswordResetTokenCheckInput, output = PasswordResetTokenCheckOutput, scope = UsersScope)]
pub struct PasswordResetTokenCheck;

#[derive(serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct PasswordResetTokenCheckInput {
    token: String,
}

#[derive(serde::Serialize, GetId, GetCategory, GetSessionToken)]
pub struct PasswordResetTokenCheckOutput {
    is_valid: bool,
}

#[derive(Default, Endpoint)]
#[internal(path = "/is_admin", input = IsAdminInput, output = bool, scope = UsersScope)]
pub struct IsAdmin;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct IsAdminInput {
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/get_admin_user_list", input = GetAdminUserListInput, output = Vec<AdminUserView>, scope = UsersScope, required = RequiredPermission::Admin, request = RequestMethod::Post)]
pub struct GetAdminUserList;

#[derive(Default, serde::Serialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminUserView {
    #[internal(id)]
    id: i32,
    pub username: String,
    pub email: String,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_active: bool,
    pub is_verified: bool,
    pub player_id: Option<i32>,
}

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct GetAdminUserListInput {
    #[internal(id)]
    user_id: i32,
    #[internal(session_token)]
    session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_user_insert", input = AdminUserInsertInput, scope = UsersScope, required = RequiredPermission::Admin, request = RequestMethod::Put)]
pub struct AdminUserInsert;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminUserInsertInput {
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_staff: bool,
    pub is_active: bool,
    pub is_verified: bool,
    pub player_id: Option<i32>,
    #[internal(session_token)]
    pub session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_user_edit", input = AdminUserEditInput, scope = UsersScope, required = RequiredPermission::Admin, request = RequestMethod::Patch)]
pub struct AdminUserEdit;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminUserEditInput {
    #[internal(id)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_staff: bool,
    pub is_active: bool,
    pub is_verified: bool,
    pub player_id: Option<i32>,
    #[internal(session_token)]
    pub session_token: String,
}

#[derive(Default, Endpoint)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[internal(path = "/admin_user_delete", input = AdminUserDeleteInput, scope = UsersScope, required = RequiredPermission::Admin, request = RequestMethod::Delete)]
pub struct AdminUserDelete;

#[derive(Default, serde::Deserialize, GetId, GetCategory, GetSessionToken)]
pub struct AdminUserDeleteInput {
    #[internal(id)]
    id: i32,
    #[internal(session_token)]
    session_token: String,
}
