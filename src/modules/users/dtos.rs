use scrutiny::Validate;
use serde::Deserialize;

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
