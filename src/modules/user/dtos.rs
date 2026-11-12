use crate::models::users;
use chrono::NaiveDateTime;
use scrutiny::Validate;
use sea_orm::prelude::Uuid;
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
    pub phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<users::Model> for UserResponseDto {
    fn from(model: users::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            password: model.password,
            first_name: model.first_name,
            last_name: model.last_name,
            phone_number: model.phone_number,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
