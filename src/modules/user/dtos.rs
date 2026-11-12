use scrutiny::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateUserDto {
    #[validate(required, email)]
    pub email: String,
    #[validate(required, min = 8)]
    pub password: String,
    #[validate(required, max = 50)]
    pub first_name: String,
    #[validate(nullable, max = 50)]
    pub last_name: Option<String>,
    #[validate(nullable, min = 10)]
    pub phone_number: Option<String>
}
