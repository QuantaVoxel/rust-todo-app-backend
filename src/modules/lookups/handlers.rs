use salvo::oapi::endpoint;
use salvo::prelude::*;
use sea_orm::{EntityTrait, QueryOrder};

use crate::common::response::ApiResponse;
use crate::core::cache::app_cache;
use crate::core::db::connection::try_get_db_connection;
use crate::entity::{external_apps, task_priorities, task_statuses};
use crate::error::api_error::ApiError;

#[endpoint(tags("Lookups"), summary = "Get list of task statuses (cached)")]
pub async fn get_statuses(_req: &mut Request, res: &mut Response) {
    if let Some(cached) = app_cache().get_statuses().await {
        res.render(Json(ApiResponse::new(cached)));
        return;
    }

    let Some(db) = try_get_db_connection() else {
        res.render(ApiError::Internal("Database belum terhubung".into()));
        return;
    };

    match task_statuses::Entity::find().all(db).await {
        Ok(statuses) => {
            app_cache().set_statuses(statuses.clone()).await;
            res.render(Json(ApiResponse::new(statuses)));
        }
        Err(err) => res.render(ApiError::Database(err)),
    }
}

#[endpoint(tags("Lookups"), summary = "Get list of task priorities (cached)")]
pub async fn get_priorities(_req: &mut Request, res: &mut Response) {
    if let Some(cached) = app_cache().get_priorities().await {
        res.render(Json(ApiResponse::new(cached)));
        return;
    }

    let Some(db) = try_get_db_connection() else {
        res.render(ApiError::Internal("Database belum terhubung".into()));
        return;
    };

    match task_priorities::Entity::find()
        .order_by_asc(task_priorities::Column::RankOrder)
        .all(db)
        .await
    {
        Ok(priorities) => {
            app_cache().set_priorities(priorities.clone()).await;
            res.render(Json(ApiResponse::new(priorities)));
        }
        Err(err) => res.render(ApiError::Database(err)),
    }
}

#[endpoint(tags("Lookups"), summary = "Get supported external applications (cached)")]
pub async fn get_external_apps(_req: &mut Request, res: &mut Response) {
    if let Some(cached) = app_cache().get_external_apps().await {
        res.render(Json(ApiResponse::new(cached)));
        return;
    }

    let Some(db) = try_get_db_connection() else {
        res.render(ApiError::Internal("Database belum terhubung".into()));
        return;
    };

    match external_apps::Entity::find().all(db).await {
        Ok(apps) => {
            app_cache().set_external_apps(apps.clone()).await;
            res.render(Json(ApiResponse::new(apps)));
        }
        Err(err) => res.render(ApiError::Database(err)),
    }
}

#[endpoint(tags("Lookups"), summary = "Get all enum reference values")]
pub async fn get_enums(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(serde_json::json!({
        "vote_type": ["up", "down"],
        "frequency": ["daily", "weekly", "monthly", "yearly"],
        "permission": ["view", "comment", "edit"],
        "connection_status": ["pending", "accepted", "blocked"],
        "notification_type": ["reminder", "comment", "share", "system"]
    }))));
}
