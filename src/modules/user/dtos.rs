use serde::{Deserialize, Serialize};
use scrutiny::Validate;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateUser {
    #[validate(required,email(message = "Format email tidak valid"))]
    pub email: String,
    #[validate(required,min(value = 1, message = "Password tidak boleh kosong"))]
    pub password: String,
    #[validate(required,min(value = 1, message = "Nama depan tidak boleh kosong"))]
    pub first_name: String,
    pub last_name: Option<String>,
}
