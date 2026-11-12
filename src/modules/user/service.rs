use crate::core::errors::ApiError;
use crate::models::users::{self, ActiveModel as UserActiveModel};
use crate::modules::user::dtos::CreateUserDto;
use salvo::prelude::*;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::collections::BTreeMap;


pub async fn create_user(
    db: &DatabaseConnection,
    payload: CreateUserDto,
) -> Result<Option<users::Model>, ApiError> {
    let password_clone = payload.password.clone();
    let hashed_password = tokio::task::spawn_blocking(move || bcrypt::hash(password_clone, 10))
        .await
        .map_err(|_| ApiError::internal_server_error("Thread pool error"))?
        .map_err(|_| ApiError::internal_server_error("Gagal enkripsi password"))?;

    let new_user = UserActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(payload.email),
        password: Set(hashed_password),
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        phone_number: Set(payload.phone_number),
        ..Default::default()
    };

    match new_user.insert(db).await {
        Ok(user) => Ok(Option::from(user)),
        Err(error) => {
            let err_msg = error.to_string().to_lowercase();

            if err_msg.contains("duplicate key") || err_msg.contains("unique constraint") {
                let mut errors = BTreeMap::new();
                errors.insert(
                    "email".to_string(),
                    vec!["Email ini sudah terdaftar.".to_string()],
                );

                return Err(ApiError {
                    message: "Validasi Database Gagal".into(),
                    errors,
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                });
            }

            Err(ApiError::internal_server_error(
                "Gagal menyimpan data pengguna ke database.",
            ))
        }
    }
}
