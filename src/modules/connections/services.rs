use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::core::db::connection::try_get_db_connection;
use crate::entity::sea_orm_active_enums::ConnectionStatusEnum;
use crate::entity::{user_connections, users};
use crate::error::api_error::ApiError;
use crate::modules::connections::dtos::*;

pub struct ConnectionService;

impl ConnectionService {
    pub async fn list_connections(
        user_id: i32,
        status_filter: Option<String>,
    ) -> Result<Vec<user_connections::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let mut query = user_connections::Entity::find().filter(
            Condition::any()
                .add(user_connections::Column::UserId1.eq(user_id))
                .add(user_connections::Column::UserId2.eq(user_id)),
        );

        if let Some(ref s) = status_filter {
            let status = match s.as_str() {
                "pending" => Some(ConnectionStatusEnum::Pending),
                "accepted" => Some(ConnectionStatusEnum::Accepted),
                "blocked" => Some(ConnectionStatusEnum::Blocked),
                _ => None,
            };
            if let Some(st) = status {
                query = query.filter(user_connections::Column::Status.eq(st));
            }
        }

        let items = query
            .order_by_desc(user_connections::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_connection(
        user_id: i32,
        dto: CreateConnectionRequest,
    ) -> Result<user_connections::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let target_user_id = dto
            .target_user_id
            .ok_or_else(|| ApiError::BadRequest("target_user_id wajib diisi".into()))?;

        if target_user_id == user_id {
            return Err(ApiError::BadRequest(
                "Tidak dapat membuat koneksi dengan diri sendiri".into(),
            ));
        }

        let target_exists = users::Entity::find_by_id(target_user_id)
            .one(db)
            .await?
            .is_some();
        if !target_exists {
            return Err(ApiError::NotFound("Pengguna tujuan tidak ditemukan".into()));
        }

        let existing = user_connections::Entity::find()
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(user_connections::Column::UserId1.eq(user_id))
                            .add(user_connections::Column::UserId2.eq(target_user_id)),
                    )
                    .add(
                        Condition::all()
                            .add(user_connections::Column::UserId1.eq(target_user_id))
                            .add(user_connections::Column::UserId2.eq(user_id)),
                    ),
            )
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(ApiError::Conflict(
                "Koneksi antara kedua user sudah ada atau sedang menunggu".into(),
            ));
        }

        let new_conn = user_connections::ActiveModel {
            user_id_1: Set(user_id),
            user_id_2: Set(target_user_id),
            status: Set(ConnectionStatusEnum::Pending),
            created_at: Set(Utc::now().into()),
            ..Default::default()
        };

        let created = new_conn.insert(db).await?;
        Ok(created)
    }

    pub async fn update_connection(
        user_id: i32,
        connection_id: i32,
        dto: UpdateConnectionRequest,
    ) -> Result<user_connections::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let conn = user_connections::Entity::find_by_id(connection_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Koneksi tidak ditemukan".into()))?;

        if conn.user_id_1 != user_id && conn.user_id_2 != user_id {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah koneksi ini".into(),
            ));
        }

        let status_str = dto
            .status
            .ok_or_else(|| ApiError::BadRequest("status wajib diisi".into()))?;
        let new_status = match status_str.as_str() {
            "accepted" => ConnectionStatusEnum::Accepted,
            "blocked" => ConnectionStatusEnum::Blocked,
            _ => {
                return Err(ApiError::BadRequest(
                    "Status koneksi harus 'accepted' atau 'blocked'".into(),
                ))
            }
        };

        let mut active: user_connections::ActiveModel = conn.into();
        active.status = Set(new_status);
        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn delete_connection(user_id: i32, connection_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let conn = user_connections::Entity::find_by_id(connection_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Koneksi tidak ditemukan".into()))?;

        if conn.user_id_1 != user_id && conn.user_id_2 != user_id {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus koneksi ini".into(),
            ));
        }

        user_connections::Entity::delete_by_id(connection_id)
            .exec(db)
            .await?;
        Ok(())
    }
}
