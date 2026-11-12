use crate::core::errors::ApiError;
use crate::core::filter::{FilterQueryDto, build_dynamic_sort};
use crate::core::pagination::{PaginationMeta, fetch_pagination};
use crate::models::prelude::Users;
use crate::models::users::{self, ActiveModel as UserActiveModel};
use crate::modules::user::dtos::CreateUserDto;
use salvo::prelude::*;
use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, Set,
};
use sea_orm::{ColumnTrait, QueryFilter};
use std::collections::BTreeMap;

pub async fn get_all_users(
    db: &DatabaseConnection,
    filter: FilterQueryDto,
) -> Result<(Vec<users::Model>, PaginationMeta), ApiError> {
    let mut query = Users::find();

    if let Some(keyword) = filter.search {
        let search_pattern = format!("%{}%", keyword);

        query = query.filter(
            Condition::any()
                .add(users::Column::Email.ilike(&search_pattern))
                .add(users::Column::FirstName.ilike(&search_pattern))
                .add(users::Column::LastName.ilike(&search_pattern))
                .add(users::Column::PhoneNumber.ilike(&search_pattern)),
        );
    }

    let allowed_sort_columns = &[
        "first_name",
        "last_name",
        "email",
        "created_at"
    ];

    let (order_column, order_direction) = build_dynamic_sort(
        filter.order_by.as_ref(),
        filter.order_dir.as_ref(),
        users::Column::CreatedAt,
        allowed_sort_columns,
    );

    query = query.order_by(order_column, order_direction);

    let limit = filter.limit.unwrap_or(50);
    let page = filter.page.unwrap_or(1); 

    let paginator = query.paginate(db, limit);

    fetch_pagination(paginator, page, limit).await
}

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
