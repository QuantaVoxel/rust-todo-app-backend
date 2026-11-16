use salvo::oapi::endpoint;
use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::modules::notifications::dtos::*;
use crate::modules::notifications::services::NotificationService;

#[endpoint(tags("Notifications"), summary = "List notifications with optional is_read filter", security(("bearer_auth" = [])))]
pub async fn list_notifications(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let is_read_filter = req.query::<bool>("is_read");

    match NotificationService::list_notifications(user.user_id, is_read_filter).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Notifications"), summary = "Get unread notifications count", security(("bearer_auth" = [])))]
pub async fn get_unread_count(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match NotificationService::get_unread_count(user.user_id).await {
        Ok(count) => res.render(Json(ApiResponse::new(serde_json::json!({
            "unread_count": count
        })))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Notifications"), summary = "Mark single notification as read or unread", security(("bearer_auth" = [])))]
pub async fn mark_notification_read(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let id = req.param::<i32>("notification_id").unwrap_or_default();
    let dto = match parse_and_validate::<UpdateNotificationRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match NotificationService::mark_notification_read(user.user_id, id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Notifications"), summary = "Mark all notifications as read", security(("bearer_auth" = [])))]
pub async fn mark_all_read(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match NotificationService::mark_all_read(user.user_id).await {
        Ok(_) => res.render(Json(ApiResponse::new(
            "Semua notifikasi telah ditandai dibaca",
        ))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Notifications"), summary = "Delete notification by ID", security(("bearer_auth" = [])))]
pub async fn delete_notification(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let id = req.param::<i32>("notification_id").unwrap_or_default();

    match NotificationService::delete_notification(user.user_id, id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Notifikasi berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}
