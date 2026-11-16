use salvo::prelude::*;
use sea_orm::{EntityTrait, QueryOrder};

use crate::common::response::ApiResponse;
use crate::core::db::connection::try_get_db_connection;
use crate::entity::{external_apps, task_priorities, task_statuses};
use crate::error::api_error::ApiError;

#[handler]
pub async fn get_statuses(_req: &mut Request, res: &mut Response) {
    let Some(db) = try_get_db_connection() else {
        res.render(ApiError::Internal("Database belum terhubung".into()));
        return;
    };

    match task_statuses::Entity::find().all(db).await {
        Ok(statuses) => res.render(Json(ApiResponse::new(statuses))),
        Err(err) => res.render(ApiError::Database(err)),
    }
}

#[handler]
pub async fn get_priorities(_req: &mut Request, res: &mut Response) {
    let Some(db) = try_get_db_connection() else {
        res.render(ApiError::Internal("Database belum terhubung".into()));
        return;
    };

    match task_priorities::Entity::find()
        .order_by_asc(task_priorities::Column::RankOrder)
        .all(db)
        .await
    {
        Ok(priorities) => res.render(Json(ApiResponse::new(priorities))),
        Err(err) => res.render(ApiError::Database(err)),
    }
}

#[handler]
pub async fn get_external_apps(_req: &mut Request, res: &mut Response) {
    let Some(db) = try_get_db_connection() else {
        res.render(ApiError::Internal("Database belum terhubung".into()));
        return;
    };

    match external_apps::Entity::find().all(db).await {
        Ok(apps) => res.render(Json(ApiResponse::new(apps))),
        Err(err) => res.render(ApiError::Database(err)),
    }
}

#[handler]
pub async fn get_enums(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(serde_json::json!({
        "vote_type": ["up", "down"],
        "frequency": ["daily", "weekly", "monthly", "yearly"],
        "permission": ["view", "comment", "edit"],
        "connection_status": ["pending", "accepted", "blocked"],
        "notification_type": ["reminder", "comment", "share", "system"]
    }))));
}
