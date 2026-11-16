use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::core::db::connection::try_get_db_connection;
use crate::entity::user_notifications;
use crate::error::api_error::ApiError;
use crate::modules::notifications::dtos::*;

pub struct NotificationService;

impl NotificationService {
    pub async fn list_notifications(
        user_id: i32,
        is_read_filter: Option<bool>,
    ) -> Result<Vec<user_notifications::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let mut query = user_notifications::Entity::find()
            .filter(user_notifications::Column::UserId.eq(user_id));

        if let Some(read) = is_read_filter {
            query = query.filter(user_notifications::Column::IsRead.eq(read));
        }

        let items = query
            .order_by_desc(user_notifications::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn get_unread_count(user_id: i32) -> Result<i64, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let count = user_notifications::Entity::find()
            .filter(user_notifications::Column::UserId.eq(user_id))
            .filter(user_notifications::Column::IsRead.eq(false))
            .count(db)
            .await? as i64;
        Ok(count)
    }

    pub async fn mark_notification_read(
        user_id: i32,
        notification_id: i32,
        dto: UpdateNotificationRequest,
    ) -> Result<user_notifications::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let notif = user_notifications::Entity::find_by_id(notification_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Notifikasi tidak ditemukan".into()))?;

        if notif.user_id != user_id {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengakses notifikasi ini".into(),
            ));
        }

        let is_read = dto
            .is_read
            .ok_or_else(|| ApiError::BadRequest("is_read wajib diisi".into()))?;

        let mut active: user_notifications::ActiveModel = notif.into();
        active.is_read = Set(is_read);
        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn mark_all_read(user_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        user_notifications::Entity::update_many()
            .col_expr(
                user_notifications::Column::IsRead,
                sea_orm::sea_query::Expr::value(true),
            )
            .filter(user_notifications::Column::UserId.eq(user_id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn delete_notification(user_id: i32, notification_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let notif = user_notifications::Entity::find_by_id(notification_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Notifikasi tidak ditemukan".into()))?;

        if notif.user_id != user_id {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus notifikasi ini".into(),
            ));
        }

        user_notifications::Entity::delete_by_id(notification_id)
            .exec(db)
            .await?;
        Ok(())
    }
}
