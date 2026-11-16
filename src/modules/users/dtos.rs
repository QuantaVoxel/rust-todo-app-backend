use chrono::{DateTime, Utc};
use scrutiny::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(sometimes, min = 3, max = 50)]
    pub username: Option<String>,

    #[validate(sometimes, email)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpsertProfileRequest {
    #[validate(sometimes, max = 255)]
    pub first_name: Option<String>,

    #[validate(sometimes, max = 255)]
    pub last_name: Option<String>,

    #[validate(sometimes, url)]
    pub avatar_url: Option<String>,

    #[validate(sometimes, max = 20)]
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpsertSettingRequest {
    #[validate(required, max = 255)]
    pub setting_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub profile_id: i32,
    pub user_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMeResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub profile: Option<ProfileResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSearchResult {
    pub user_id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettingResponse {
    pub setting_id: i32,
    pub user_id: i32,
    pub setting_key: String,
    pub setting_value: Option<String>,
}
