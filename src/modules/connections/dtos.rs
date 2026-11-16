use scrutiny::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateConnectionRequest {
    #[validate(required)]
    pub target_user_id: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateConnectionRequest {
    #[validate(required, in_list("accepted", "blocked"))]
    pub status: Option<String>,
}
