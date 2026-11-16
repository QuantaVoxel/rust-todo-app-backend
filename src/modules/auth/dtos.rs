use chrono::{DateTime, Utc};
use scrutiny::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(required, min = 3, max = 50)]
    pub username: Option<String>,

    #[validate(required, email, bail)]
    pub email: Option<String>,

    #[validate(required, min = 8)]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(required, email, bail)]
    pub email: Option<String>,

    #[validate(required, min = 1)]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSummary {
    pub user_id: i32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserSummary,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(required)]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserSummary,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(required)]
    pub old_password: Option<String>,

    #[validate(required, min = 8)]
    pub new_password: Option<String>,

    pub new_password_confirmation: Option<String>,
}
