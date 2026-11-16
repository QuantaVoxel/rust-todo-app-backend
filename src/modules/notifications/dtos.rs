use scrutiny::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNotificationRequest {
    #[validate(required)]
    pub is_read: Option<bool>,
}
